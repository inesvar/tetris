//! Defines the app that handles the players, their interactions and the changes of views, settings and number of players.
mod player;
mod remote;
mod render_app;
mod update_app;

use std::net::TcpStream;

use self::player::LocalPlayer;
pub use self::player::{PlayerScreen, Tetromino};
use self::remote::RemotePlayer;
use crate::settings::*;
use crate::ui::interactive_widget_manager::InteractiveWidgetManager;
use crate::ui::text::Text;
use crate::Assets;
use crate::{app::remote::MessageType, PlayerConfig};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::MouseButton;
use piston_window::Key;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Indicates whether the player commands lead the game to pause, resume, restart or no.
/// The GameOver variant is only used for remote players.
#[derive(Debug, PartialEq)]
pub enum GameFlowChange {
    Restart,
    Resume,
    Pause,
    GameOver,
    Sync(Settings),
    Other,
}

#[derive(Debug, PartialEq)]
pub enum ViewState {
    MainMenu,
    Settings,
    //JoinRoom,
    //CreateRoom,
    SinglePlayerGame,
    //LocalMultiplayerGame,
    //OnlineMultiplayerGame,
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
    player_config: PlayerConfig,
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
    widget_manager: InteractiveWidgetManager,
    keybindings_manager: Keybindings,
    settings_manager: Settings,
    is_synchronized: bool,
    is_host: bool,
}

impl App<'_> {
    pub fn new(gl_version: OpenGL, player_config: PlayerConfig) -> App<'static> {
        let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let local_player: LocalPlayer;
        let remote_player: RemotePlayer;
        let players: Vec<LocalPlayer>;
        let rem_players: Vec<RemotePlayer>;
        let mut rng = rand::thread_rng();
        let seed: u64 = rng.gen();
        let mut is_host = false;

        match &player_config {
            PlayerConfig::Local => {
                local_player = LocalPlayer::new(seed, &player_config);
                players = vec![local_player];
                rem_players = vec![]
            }
            PlayerConfig::Streamer(_) => {
                local_player = LocalPlayer::new(seed, &player_config);
                players = vec![local_player];
                rem_players = vec![]
            }
            PlayerConfig::Viewer(local_ip) => {
                remote_player = RemotePlayer::new();
                players = vec![];
                rem_players = vec![remote_player];
                rem_players[0].listen(&local_ip)
            }
            PlayerConfig::TwoRemote {
                local_ip,
                remote_ip: _,
            } => {
                local_player = LocalPlayer::new(seed, &player_config);
                players = vec![local_player];
                remote_player = RemotePlayer::new();
                rem_players = vec![remote_player];
                rem_players[0].listen(&local_ip);
                is_host = if local_ip.chars().last().unwrap() == '0' {
                    true
                } else {
                    false
                };
            }
            _ => todo!(),
        }

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
            widget_manager: InteractiveWidgetManager::new_main_menu(),
            keybindings_manager: Keybindings::new(),
            settings_manager,
            is_synchronized: false,
            is_host,
        };
        app
    }

    pub fn handle_text_input(&mut self, input: &String) {
        match self.view_state {
            ViewState::MainMenu => self.widget_manager.handle_text_input(input),
            ViewState::Settings => self.widget_manager.handle_text_input(input),
            _ => {}
        }
    }

    pub fn handle_key_press(&mut self, key: Key) {
        let mut key_press = GameFlowChange::Other;
        for player in &mut self.local_players {
            key_press = player.handle_key_press(&self.keybindings_manager, key, self.running)
        }
        match key_press {
            GameFlowChange::Restart => self.restart(),
            GameFlowChange::Resume => self.pause(),
            GameFlowChange::Pause => self.pause(),
            GameFlowChange::GameOver => self.game_over(),
            _ => {}
        }

        match self.view_state {
            ViewState::MainMenu => self.widget_manager.handle_key_press(key),
            ViewState::Settings => self.widget_manager.handle_key_press(key),
            _ => {}
        }
    }

    pub fn handle_remote(&mut self) {
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
                    self.pause()
                }
            }
            GameFlowChange::Restart => {
                if self.running == RunningState::NotRunning {
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
            _ => {}
        }
    }

    pub fn handle_key_release(&mut self, key: Key) {
        for player in &mut self.local_players {
            player.handle_key_release(key);
        }
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton) {
        self.widget_manager
            .handle_mouse_press(button, &self.cursor_position);
    }

    pub fn handle_mouse_release(&mut self, button: MouseButton) {
        self.widget_manager.handle_mouse_release(button);
    }

    fn set_view(&mut self, view_state: ViewState) {
        println!("setting view to {:?}", view_state);
        self.view_state = view_state;
        match self.view_state {
            ViewState::MainMenu => self.widget_manager = InteractiveWidgetManager::new_main_menu(),
            ViewState::Settings => {
                self.widget_manager =
                    InteractiveWidgetManager::new_settings(&self.keybindings_manager)
            }
            ViewState::SinglePlayerGame => {
                self.widget_manager = InteractiveWidgetManager::new_single_player_game()
            }
            #[allow(unreachable_patterns)]
            _ => self.widget_manager = InteractiveWidgetManager::new_empty(),
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
    /// Starts a countdown then starts the game.
    fn restart(&mut self) {
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
            PlayerConfig::Streamer(remote_ip) => {
                if let Ok(stream) = TcpStream::connect(remote_ip) {
                    serde_cbor::to_writer::<TcpStream, MessageType>(stream, &message).unwrap();
                }
            }
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
