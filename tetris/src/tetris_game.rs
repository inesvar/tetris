//! Application that handles the menu, game logic and configuration.
mod player;
mod player_config;
mod remote;
mod render_app;
mod update_app;

use crate::settings::*;
use crate::{app::ViewState, game_flow_change::GameFlowChange, once};
use core_tetris::render::RunningState;
use local_ip_address::local_ip;
use piston_window::Key;
use player::LocalPlayer;
pub use player::TetrisPlayer;
pub(super) use player_config::PlayerConfig;
use rand::RngExt;
pub(super) use remote::OutboundMessage;
use remote::RemotePlayer;
pub use render_app::RenderTetrisGame;
use std::cell::RefCell;
use std::net::TcpStream;
use ui_tetris::{Keybindings, GUEST_PORT, HOST_PORT};

pub struct TetrisGame {
    local_players: Vec<LocalPlayer>, // TODO: what about storing the first two in the third ?
    remote_player: Vec<RemotePlayer>, // TODO: what about storing the first two in the third ?
    pub player_config: PlayerConfig, // TODO: what about storing the first two in the third ?
    running: RunningState,
    tcp_stream: RefCell<Option<TcpStream>>, // TODO: there should be a kind of "network manager handling this"
    is_synchronized: bool, // TODO: there should be a kind of "network manager handling this"
    is_host: bool,         // TODO: there should be a kind of "network manager handling this"
    pub clock: f64,
    settings_manager: Settings,
}

impl TetrisGame {
    pub(super) fn new() -> Self {
        let mut rng = rand::rng();
        let seed: u64 = rng.random();
        let is_host = false;
        let player_config = PlayerConfig::Local;

        let local_player: LocalPlayer = LocalPlayer::new(Keybindings::new_local());
        let players: Vec<LocalPlayer> = vec![local_player];
        let rem_players: Vec<RemotePlayer> = vec![];

        let settings_manager = Settings::new(seed);

        TetrisGame {
            local_players: players,
            remote_player: rem_players,
            player_config,
            running: RunningState::NotRunning,
            tcp_stream: RefCell::new(None),
            is_synchronized: false,
            is_host,
            settings_manager,
            clock: 0.0,
        }
    }

    pub(super) fn mut_local_players(&mut self) -> &mut Vec<LocalPlayer> {
        &mut self.local_players
    }

    pub(super) fn keybindings(&self) -> impl Iterator<Item = Keybindings> + use<'_> {
        self.local_players
            .iter()
            .map(|player| player.get_keybindings())
    }

    pub fn set_player_config(&mut self, player_config: PlayerConfig) {
        // make the game unactive
        self.running = RunningState::NotRunning;
        // kill the previous listener
        if let PlayerConfig::TwoRemote {
            local_ip: _,
            remote_ip: _,
        } = self.player_config
        {
            let server: String = if self.is_host {
                local_ip().unwrap().to_string() + HOST_PORT
            } else {
                local_ip().unwrap().to_string() + GUEST_PORT
            };
            //let local_ip = "127.0.0.1".to_string() + HOST_PORT;
            if let Ok(stream) = TcpStream::connect(server) {
                serde_cbor::to_writer::<TcpStream, OutboundMessage>(stream, &OutboundMessage::Kill)
                    .unwrap();
            }
        }

        println!("SETTING PLAYER CONFIG {:?}", player_config);
        let local_player: LocalPlayer;
        let remote_player: RemotePlayer;

        match &player_config {
            PlayerConfig::Local => {
                local_player = LocalPlayer::new(Keybindings::new_local());
                self.local_players = vec![local_player];
                self.remote_player = vec![];
            }
            PlayerConfig::Viewer(local_ip) => {
                self.local_players = vec![];
                if self.remote_player.is_empty() {
                    remote_player = RemotePlayer::new();
                    self.remote_player = vec![remote_player];
                    self.remote_player[0].listen(local_ip);
                }
            }
            PlayerConfig::TwoRemote {
                local_ip,
                remote_ip,
            } => {
                local_player = LocalPlayer::new(Keybindings::new_local());
                self.local_players = vec![local_player];
                self.tcp_stream = RefCell::new(Some(
                    TcpStream::connect(remote_ip).expect("Couldn't open tcp stream !"),
                ));
                println!("Opened TCP stream");
                if self.remote_player.is_empty() {
                    remote_player = RemotePlayer::new();
                    self.remote_player = vec![remote_player];
                    self.remote_player[0].listen(local_ip);
                }
                self.is_host = local_ip.ends_with('0');
            }
            PlayerConfig::TwoLocal => {
                local_player = LocalPlayer::new(Keybindings::new_two_local(0));
                let second_local = LocalPlayer::new(Keybindings::new_two_local(1));
                self.local_players = vec![local_player, second_local];
                self.remote_player = vec![];
            }
        }

        self.player_config = player_config;
    }

    pub(super) fn handle_key_press_in_game(&mut self, key: Key) -> GameFlowChange {
        if self.running == RunningState::NotRunning && RESTART_KEYS.contains(&key) {
            GameFlowChange::Restart
        } else if self.running == RunningState::Paused && PAUSE_KEYS.contains(&key) {
            GameFlowChange::Resume
        } else if self.running == RunningState::Running && PAUSE_KEYS.contains(&key) {
            GameFlowChange::Pause
        } else {
            if self.running == RunningState::Running {
                for player in self.local_players.iter_mut() {
                    player.handle_key_press(key)
                }
            }
            GameFlowChange::NoChange
        }
    }

    pub(super) fn update_game_flow(&mut self, game_flow_change: GameFlowChange) {
        match game_flow_change {
            GameFlowChange::Restart => {
                if self.running == RunningState::NotRunning {
                    self.restart()
                }
            }
            GameFlowChange::Resume => {
                if self.running == RunningState::Paused {
                    self.pause()
                }
            }
            GameFlowChange::Pause => {
                if self.running == RunningState::Running {
                    self.pause()
                }
            }
            GameFlowChange::GameOver => {
                if self.running == RunningState::Running {
                    self.game_over()
                }
            }
            _ => {}
        }
    }

    pub(super) fn handle_remote(&mut self) -> Option<ViewState> {
        once!("handle remote was called");
        let mut game_flow_change: GameFlowChange = GameFlowChange::NoChange;
        for player in &self.remote_player {
            // supposing there's only one player
            // TODO : change this for multiple remote players
            game_flow_change = player.get_game_flow();
        }
        match game_flow_change {
            GameFlowChange::GameOver | GameFlowChange::Pause => {
                self.update_game_flow(game_flow_change);
                None
            }
            GameFlowChange::Resume | GameFlowChange::Restart => {
                self.update_game_flow(game_flow_change);
                Some(ViewState::Game(2))
            }
            GameFlowChange::Sync(new_settings) => {
                self.settings_manager = new_settings;
                for player in &mut self.local_players {
                    player.reset(self.settings_manager.seed);
                }
                self.is_synchronized = true;
                if self.is_host {
                    self.restart()
                } else {
                    self.send_message(OutboundMessage::Settings(&self.settings_manager));
                }
                None
            }
            GameFlowChange::Hello(remote_ip) => {
                let local_ip = local_ip().unwrap().to_string() + HOST_PORT;
                //let local_ip = "127.0.0.1".to_string() + HOST_PORT;
                let player_config = PlayerConfig::TwoRemote {
                    local_ip,
                    remote_ip,
                };
                self.set_player_config(player_config);
                self.send_message(OutboundMessage::TetrisPlayer((
                    self.local_players[0].player_data(),
                    0,
                )));
                Some(ViewState::Game(2))
            }
            GameFlowChange::NoChange => None,
        }
    }

    pub fn handle_key_release(&mut self, key: Key) {
        for player in &mut self.local_players {
            player.handle_key_release(key);
        }
    }

    fn pause(&mut self) {
        if self.running == RunningState::Paused {
            println!("RESUME");
            self.send_message(OutboundMessage::Resume);
            self.running = RunningState::Running;
        } else if self.running == RunningState::Running {
            println!("PAUSE");
            self.send_message(OutboundMessage::Pause);
            self.running = RunningState::Paused;
        }
    }

    /// Starts a countdown then starts the game. Inn TwoRemote mode, sends a synchronizing message.
    pub(super) fn restart(&mut self) {
        match &self.player_config {
            PlayerConfig::TwoRemote {
                local_ip: _,
                remote_ip: _,
            } => {
                if self.is_synchronized {
                    println!("RESTART");
                    self.send_message(OutboundMessage::Restart);
                    self.running = RunningState::Starting;
                    self.clock = 0.0;
                } else if self.is_host {
                    println!("HOST SYNCHRONIZE");
                    let mut rng = rand::rng();
                    self.settings_manager.seed = rng.random();
                    self.send_message(OutboundMessage::Settings(&self.settings_manager));
                } else {
                    self.send_message(OutboundMessage::Restart);
                }
            }
            PlayerConfig::TwoLocal => {
                self.running = RunningState::Starting;
                self.clock = 0.0;
                let mut rng = rand::rng();
                self.settings_manager.seed = rng.random();
                for player in &mut self.local_players {
                    player.reset(self.settings_manager.seed);
                }
            }
            PlayerConfig::Local => {
                self.running = RunningState::Starting;
                self.clock = 0.0;
                let mut rng = rand::rng();
                self.settings_manager.seed = rng.random();
                for player in &mut self.local_players {
                    player.reset(self.settings_manager.seed);
                }
            }
            _ => {
                self.running = RunningState::Starting;
                self.clock = 0.0;
            }
        }
    }

    /// Makes the game active.
    fn start(&mut self) {
        self.clock = 0.0;
        self.running = RunningState::Running;
    }

    /// Makes the game unactive.
    fn game_over(&mut self) {
        println!("GAMEOVER");
        self.send_message(OutboundMessage::GameOver);
        self.running = RunningState::NotRunning;
        self.is_synchronized = false;
    }

    pub(super) fn send_message(&self, message: OutboundMessage) {
        // Try existing connection.
        if let Some(stream) = self.tcp_stream.borrow().as_ref() {
            if serde_cbor::to_writer(stream, &message).is_ok() {
                return;
            }

            println!("Existing connection died.");
        }

        // Reconnect.
        let remote_ip = match &self.player_config {
            PlayerConfig::TwoRemote { remote_ip, .. } => remote_ip,
            _ => return,
        };

        match TcpStream::connect(remote_ip) {
            Ok(stream) => {
                println!("Reconnected.");
                *self.tcp_stream.borrow_mut() = Some(stream);

                if let Some(stream) = self.tcp_stream.borrow().as_ref() {
                    let _ = serde_cbor::to_writer(stream, &message);
                }
            }
            Err(e) => {
                println!("Reconnect failed: {:?}", e);
            }
        }
    }
}
