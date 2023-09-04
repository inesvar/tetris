//! Defines the app that handles the players, their interactions and the changes of views, settings and number of players.
mod player;
mod remote;
mod render_app;
mod update_app;

use self::player::LocalPlayer;
pub use self::player::{PlayerScreen, Tetromino};
use self::remote::RemotePlayer;
use crate::ui::{
    interactive_widget_manager::{InteractiveWidgetManager, SettingsType},
    text::Text,
};
use crate::Assets;
use crate::{app::remote::MessageType, PlayerConfig};
use crate::{once, settings::*};
use local_ip_address::local_ip;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::MouseButton;
use piston_window::Key;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::net::TcpStream;

/// Indicates whether the player commands lead the game to pause, resume, restart or no.
/// The GameOver variant is only used for remote players.
#[derive(Debug, PartialEq)]
pub enum GameFlowChange {
    Restart,
    Resume,
    Pause,
    GameOver,
    Sync(Settings),
    Hello(String),
    Other,
}

/// View state indicates what is on screen.
/// The game states are handled differently with help of the [is_game()] method.
#[derive(Debug, PartialEq)]
pub enum ViewState {
    MainMenu,
    Settings,
    JoinRoom,
    CreateRoom,
    Local,
    TwoLocal,
    Remote,
}

impl ViewState {
    fn is_game(&self) -> bool {
        match &self {
            Self::Local => true,
            Self::TwoLocal => true,
            Self::Remote => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum RunningState {
    Running,
    Paused,
    NotRunning,
    Starting,
}

pub struct App<'a> {
    gl: GlGraphics,
    local_players: Vec<LocalPlayer>,
    remote_player: Vec<RemotePlayer>,
    pub player_config: PlayerConfig,
    view_state: ViewState,
    assets: Assets<'a>,
    pub clock: f64,
    frame_counter: u64,
    running: RunningState,
    title_text: Text,
    restart_text: Text,
    pause_text: Text,
    timer_text: Text,
    pub cursor_position: [f64; 2],
    widget_manager: Vec<InteractiveWidgetManager>,
    keybindings_manager: Vec<Keybindings>,
    settings_manager: Settings,
    is_synchronized: bool,
    is_host: bool,
}

impl App<'_> {
    pub fn new(gl_version: OpenGL) -> App<'static> {
        let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let local_player: LocalPlayer;
        let players: Vec<LocalPlayer>;
        let rem_players: Vec<RemotePlayer>;
        let mut rng = rand::thread_rng();
        let seed: u64 = rng.gen();
        let is_host = false;
        let player_config = PlayerConfig::Local;

        local_player = LocalPlayer::new(&player_config);
        players = vec![local_player];
        rem_players = vec![];

        let assets = Assets::new(assets_folder);
        let settings_manager = Settings::new(seed, &player_config);

        let app = App {
            gl: GlGraphics::new(gl_version),
            local_players: players,
            remote_player: rem_players,
            player_config,
            view_state: ViewState::MainMenu,
            assets,
            title_text: Text::new(
                "T",
                DEFAULT_FONT_SIZE,
                DEFAULT_WINDOW_WIDTH as f64 * 27.0 / 65.0,
                DEFAULT_TITLE_Y,
                TEXT_COLOR,
            ),
            restart_text: Text::new(
                "Press R to (re)start",
                (DEFAULT_FONT_SIZE * 22) / 16,
                DEFAULT_WINDOW_WIDTH as f64 / 2.0,
                DEFAULT_TITLE_Y,
                TEXT_COLOR,
            ),
            timer_text: Text::new(
                "Elapsed: 0.0s",
                DEFAULT_FONT_SIZE,
                DEFAULT_GRID_X - 4.0 * BLOCK_SIZE,
                DEFAULT_SCORE_TEXT_Y + 1.5 * BLOCK_SIZE,
                TEXT_COLOR,
            ),
            pause_text: Text::new(
                "Press P to resume",
                (DEFAULT_FONT_SIZE * 22) / 16,
                DEFAULT_WINDOW_WIDTH as f64 / 2.0,
                DEFAULT_TITLE_Y,
                TEXT_COLOR,
            ),
            clock: 0.0,
            frame_counter: 0,
            running: RunningState::NotRunning,
            cursor_position: [0.0, 0.0],
            widget_manager: vec![InteractiveWidgetManager::new_main_menu()],
            keybindings_manager: vec![Keybindings::new()],
            settings_manager,
            is_synchronized: false,
            is_host,
        };
        app
    }

    pub fn set_player_config(&mut self, player_config: PlayerConfig) {
        // make the game unactive
        self.running = RunningState::NotRunning;
        // kill the previous listener
        match self.player_config {
            PlayerConfig::TwoRemote {
                local_ip: _,
                remote_ip: _,
            } => {
                let server: String;
                if self.is_host {
                    server = local_ip().unwrap().to_string() + HOST_PORT;
                } else {
                    server = local_ip().unwrap().to_string() + GUEST_PORT;
                }
                //let local_ip = "127.0.0.1".to_string() + HOST_PORT;
                if let Ok(stream) = TcpStream::connect(server) {
                    serde_cbor::to_writer::<TcpStream, MessageType>(stream, &MessageType::KillMsg)
                        .unwrap();
                }
            }
            _ => {}
        }

        println!("SETTING PLAYER CONFIG {:?}", player_config);
        let local_player: LocalPlayer;
        let remote_player: RemotePlayer;

        match &player_config {
            PlayerConfig::Local => {
                local_player = LocalPlayer::new(&player_config);
                self.local_players = vec![local_player];
                self.remote_player = vec![]
            }
            PlayerConfig::Viewer(local_ip) => {
                remote_player = RemotePlayer::new();
                self.local_players = vec![];
                self.remote_player = vec![remote_player];
                self.remote_player[0].listen(&local_ip)
            }
            PlayerConfig::TwoRemote {
                local_ip,
                remote_ip: _,
            } => {
                local_player = LocalPlayer::new(&player_config);
                self.local_players = vec![local_player];
                if self.remote_player.len() == 0 {
                    remote_player = RemotePlayer::new();
                    self.remote_player = vec![remote_player];
                    self.remote_player[0].listen(&local_ip);
                }
                self.is_host = if local_ip.chars().last().unwrap() == '0' {
                    true
                } else {
                    false
                };
            }
            PlayerConfig::TwoLocal => {
                local_player = LocalPlayer::new(&player_config);
                let second_local = LocalPlayer::new(&player_config);
                self.local_players = vec![local_player, second_local];
                self.remote_player = vec![]
            }
        }

        self.settings_manager.set_player_config(&player_config);
        self.player_config = player_config;
    }

    pub fn handle_text_input(&mut self, input: &String) {
        match self.view_state {
            ViewState::MainMenu => self.widget_manager[0].handle_text_input(input),
            ViewState::JoinRoom => self.widget_manager[0].handle_text_input(input),
            _ => {}
        }
    }

    pub fn handle_key_press(&mut self, key: Key) {
        let mut game_key_press = GameFlowChange::Other;
        match &self.view_state {
            ViewState::MainMenu => self.widget_manager[0].handle_key_press(key),
            ViewState::Settings => {
                for widget_manager in &mut self.widget_manager {
                    widget_manager.handle_key_press(key);
                }
            }
            ViewState::JoinRoom => self.widget_manager[0].handle_key_press(key),
            a if a.is_game() => {
                for (id, player) in self.local_players.iter_mut().enumerate() {
                    game_key_press =
                        player.handle_key_press(&self.keybindings_manager[id], key, self.running)
                }
            }
            _ => {}
        }
        match game_key_press {
            GameFlowChange::Restart => self.restart(),
            GameFlowChange::Resume => self.pause(),
            GameFlowChange::Pause => self.pause(),
            GameFlowChange::GameOver => self.game_over(),
            _ => {}
        }
    }

    pub fn handle_remote(&mut self) {
        once!("handle remote was called");
        let mut game_flow_change: GameFlowChange = GameFlowChange::Other;
        for player in &self.remote_player {
            // supposing there's only one player
            // TODO : change this for multiple remote players
            game_flow_change = player.get_game_flow();
        }
        match game_flow_change {
            GameFlowChange::GameOver => {
                if self.running == RunningState::Running {
                    self.game_over()
                }
            }
            GameFlowChange::Pause => {
                if self.running == RunningState::Running {
                    self.pause()
                }
            }
            GameFlowChange::Resume => {
                if self.running == RunningState::Paused {
                    self.set_view(ViewState::Remote);
                    self.pause()
                }
            }
            GameFlowChange::Restart => {
                once!("restart was received");
                if self.running == RunningState::NotRunning {
                    self.set_view(ViewState::Remote);
                    self.restart()
                }
            }
            GameFlowChange::Sync(new_settings) => {
                self.settings_manager.seed = new_settings.seed;
                self.settings_manager.bag_size = new_settings.bag_size;
                self.settings_manager.nb_next_tetromino = new_settings.nb_next_tetromino;
                for player in &mut self.local_players {
                    player.renew(new_settings.seed);
                }
                self.is_synchronized = true;
                if self.is_host {
                    self.restart()
                } else {
                    self.settings_manager.send();
                }
            }
            GameFlowChange::Hello(remote_ip) => {
                let local_ip = local_ip().unwrap().to_string() + HOST_PORT;
                //let local_ip = "127.0.0.1".to_string() + HOST_PORT;
                let player_config = PlayerConfig::TwoRemote {
                    local_ip,
                    remote_ip,
                };
                self.set_player_config(player_config);
                self.set_view(ViewState::Remote);
                self.local_players[0].send_serialized();
            }
            _ => {}
        }
    }

    pub fn handle_key_release(&mut self, key: Key) {
        if self.view_state.is_game() {
            for player in &mut self.local_players {
                player.handle_key_release(key);
            }
        }
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton) {
        for widget_manager in &mut self.widget_manager {
            widget_manager.handle_mouse_press(button, &self.cursor_position);
        }
    }

    pub fn handle_mouse_release(&mut self, button: MouseButton) {
        for widget_manager in &mut self.widget_manager {
            widget_manager.handle_mouse_release(button);
        }
    }

    fn set_view(&mut self, view_state: ViewState) {
        println!("setting view to {:?}", view_state);
        let from_game = self.view_state.is_game();
        self.view_state = view_state;
        match self.view_state {
            ViewState::MainMenu => {
                self.widget_manager = vec![InteractiveWidgetManager::new_main_menu()]
            }
            ViewState::Settings => {
                match &self.player_config {
                    PlayerConfig::Local => {
                        self.widget_manager = vec![InteractiveWidgetManager::new_settings(
                            &self.keybindings_manager[0],
                            SettingsType::OnePlayer,
                            from_game,
                        )]
                    }
                    _ => {
                        self.widget_manager = vec![InteractiveWidgetManager::new_settings(
                            &self.keybindings_manager[0],
                            SettingsType::LeftPlayer,
                            from_game,
                        )]
                    }
                }
                if self.player_config == PlayerConfig::TwoLocal {
                    self.keybindings_manager.push(Keybindings::new());
                    self.widget_manager
                        .push(InteractiveWidgetManager::new_settings(
                            &self.keybindings_manager[1],
                            SettingsType::RightPlayer,
                            from_game,
                        ));
                }
            }
            ViewState::TwoLocal => {
                self.keybindings_manager =
                    vec![Keybindings::new_two_local(0), Keybindings::new_two_local(1)];
                self.widget_manager = vec![InteractiveWidgetManager::new_two_player_game()];
            }
            ViewState::Local => {
                self.widget_manager = vec![InteractiveWidgetManager::new_single_player_game()];
            }
            ViewState::Remote => {
                self.widget_manager = vec![InteractiveWidgetManager::new_two_player_game()];
            }
            ViewState::CreateRoom => {
                /* let mut file = File::create("local_port.txt").unwrap();
                file.write(HOST_PORT.as_bytes()).unwrap(); */
                let local_ip = local_ip().unwrap().to_string() + HOST_PORT;
                //let local_ip = "127.0.0.1".to_string() + HOST_PORT;
                self.set_player_config(PlayerConfig::Viewer(local_ip));
                self.widget_manager = vec![InteractiveWidgetManager::new_create_room()]
            }
            ViewState::JoinRoom => {
                /* let mut file = File::create("local_port.txt").unwrap();
                file.write(GUEST_PORT.as_bytes()).unwrap(); */
                self.widget_manager = vec![InteractiveWidgetManager::new_join_room()]
            }
        }
    }

    fn pause(&mut self) {
        if self.running == RunningState::Paused {
            println!("RESUME");
            self.send_message(MessageType::ResumeMsg);
            self.running = RunningState::Running;
        } else if self.running == RunningState::Running {
            println!("PAUSE");
            self.send_message(MessageType::PauseMsg);
            self.running = RunningState::Paused;
        }
    }
    /// Starts a countdown then starts the game. Inn TwoRemote mode, sends a synchronizing message.
    fn restart(&mut self) {
        match &self.player_config {
            PlayerConfig::TwoRemote {
                local_ip: _,
                remote_ip: _,
            } => {
                if self.is_synchronized {
                    println!("RESTART");
                    self.send_message(MessageType::RestartMsg);
                    self.running = RunningState::Starting;
                    self.clock = 0.0;
                } else if self.is_host {
                    println!("HOST SYNCHRONIZE");
                    let mut rng = rand::thread_rng();
                    self.settings_manager.seed = rng.gen();
                    self.settings_manager.send();
                } else {
                    self.send_message(MessageType::RestartMsg);
                }
            }
            PlayerConfig::TwoLocal => {
                self.running = RunningState::Starting;
                self.clock = 0.0;
                let mut rng = rand::thread_rng();
                self.settings_manager.seed = rng.gen();
                for player in &mut self.local_players {
                    player.renew(self.settings_manager.seed);
                }
            }
            PlayerConfig::Local => {
                self.running = RunningState::Starting;
                self.clock = 0.0;
                let mut rng = rand::thread_rng();
                self.settings_manager.seed = rng.gen();
                for player in &mut self.local_players {
                    player.renew(self.settings_manager.seed);
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
        for player in &mut self.local_players {
            player.start();
        }
        self.clock = 0.0;
        self.running = RunningState::Running;
    }

    /// Makes the game unactive.
    fn game_over(&mut self) {
        println!("GAMEOVER");
        self.send_message(MessageType::GameOverMsg);
        self.running = RunningState::NotRunning;
        self.is_synchronized = false;
    }

    fn countdown(&mut self, i: &Countdown) {
        for player in &mut self.local_players {
            player.countdown(i);
        }
    }

    /// Sends message to the remote if there's a remote.
    fn send_message(&self, message: MessageType) {
        match &self.player_config {
            PlayerConfig::TwoRemote {
                local_ip: _,
                remote_ip,
            } => {
                if let Ok(stream) = TcpStream::connect(remote_ip) {
                    serde_cbor::to_writer::<TcpStream, MessageType>(stream, &message).unwrap();
                }
            }
            _ => {}
        }
    }
}

pub(self) enum Countdown {
    One,
    Two,
    Three,
}
