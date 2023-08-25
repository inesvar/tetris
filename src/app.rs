//! Defines the app that handles the players, their interactions and the changes of views, settings and number of players.
mod player;
mod remote;
mod render_app;
mod update_app;

use self::player::LocalPlayer;
pub use self::player::{PlayerScreen, Tetromino};
use self::remote::RemotePlayer;
use crate::settings::*;
use crate::ui::interactive_widget_manager::InteractiveWidgetManager;
use crate::ui::text::Text;
use crate::Assets;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::MouseButton;
use piston_window::Key;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Indicates whether the player commands lead the game to pause, resume, restart or no.
/// The GameOver variant is only used for remote players.
#[derive(Debug)]
pub enum GameFlowChange {
    Restart,
    Resume,
    Pause,
    GameOver,
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

pub enum PlayerConfig {
    Local,
    Streamer,
    TwoLocal,
    TwoRemote,
    Viewer,
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
    remote_players: Vec<RemotePlayer>,
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
    #[allow(dead_code)]
    settings_manager: Settings,
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

        match player_config {
            PlayerConfig::Local => {
                local_player = LocalPlayer::new(seed, false);
                players = vec![local_player];
                rem_players = vec![];
            }
            PlayerConfig::Streamer => {
                local_player = LocalPlayer::new(seed, true);
                players = vec![local_player];
                rem_players = vec![];
            }
            PlayerConfig::Viewer => {
                remote_player = RemotePlayer::new();
                players = vec![];
                rem_players = vec![remote_player];
            }
            PlayerConfig::TwoRemote => {
                local_player = LocalPlayer::new(seed, true);
                players = vec![local_player];
                remote_player = RemotePlayer::new();
                rem_players = vec![remote_player];
            }
            _ => todo!(),
        }

        let assets = Assets::new(assets_folder);

        let app = App {
            gl: GlGraphics::new(gl_version),
            local_players: players,
            remote_players: rem_players,
            player_config,
            view_state: ViewState::MainMenu,
            assets,
            title_text: Text::new(
                "T",
                16,
                DEFAULT_WINDOW_WIDTH as f64 * 27.0 / 65.0,
                DEFAULT_TITLE_Y,
                TEXT_COLOR,
            ),
            restart_text: Text::new(
                "Press R to (re)start",
                22,
                DEFAULT_WINDOW_WIDTH as f64 / 2.0,
                DEFAULT_TITLE_Y,
                TEXT_COLOR,
            ),
            timer_text: Text::new(
                "Elapsed: 0.0s",
                16,
                DEFAULT_GRID_X - 4.0 * BLOCK_SIZE,
                DEFAULT_SCORE_TEXT_Y + 1.5 * BLOCK_SIZE,
                TEXT_COLOR,
            ),
            pause_text: Text::new(
                "Press P to resume",
                16,
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
            settings_manager: Settings::new(seed),
        };

        if let PlayerConfig::Viewer = app.player_config {
            app.remote_players[0].listen()
        } else if let PlayerConfig::TwoRemote = app.player_config {
            app.remote_players[0].listen()
        }
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
            GameFlowChange::Restart => {
                self.restart();
            }
            GameFlowChange::Resume => {
                self.running = RunningState::Running;
            }
            GameFlowChange::Pause => {
                self.running = RunningState::Paused;
            }
            _ => {}
        }

        match self.view_state {
            ViewState::MainMenu => self.widget_manager.handle_key_press(key),
            ViewState::Settings => self.widget_manager.handle_key_press(key),
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
            self.running = RunningState::Running;
        } else if self.running == RunningState::Running {
            self.running = RunningState::Paused;
        }
    }
    /// Starts a countdown then starts the game.
    fn restart(&mut self) {
        self.running = RunningState::Starting;
        self.clock = 0.0;
    }

    /// Makes the game active.
    fn start(&mut self) {
        for player in &mut self.local_players {
            player.restart();
        }
        self.clock = 0.0;
        self.running = RunningState::Running;
    }

    fn countdown(&mut self, i: &Countdown) {
        for player in &mut self.local_players {
            player.countdown(i);
        }
    }
}

pub(self) enum Countdown {
    One,
    Two,
    Three,
}
