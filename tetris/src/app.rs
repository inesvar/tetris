use crate::settings::{FALL_SPEED_DIVIDE, FREEZE};
use crate::tetris_game::{PlayerConfig, TetrisGame};
use local_ip_address::local_ip;
use piston::MouseButton;
use piston_window::Key;
use ui_tetris::interactive_widget_manager::InteractiveWidgetManager;
use ui_tetris::HOST_PORT;
pub use view::ViewState;

mod view;

pub struct App {
    pub game: TetrisGame,
    pub view_state: ViewState,
    pub frame_counter: u64,
    pub cursor_position: [f64; 2],
    pub widget_manager: InteractiveWidgetManager,
    pub fall_speed_divide: u64, // TODO: that's more of a tetris engine responsibility ?
    pub freeze: u64,            // TODO: that's more of a tetris engine responsibility ?
    pub clock: f64,
}

impl App {
    pub fn new() -> Self {
        App {
            game: TetrisGame::new(),
            view_state: ViewState::MainMenu,
            frame_counter: 0,
            cursor_position: [0.0, 0.0],
            widget_manager: InteractiveWidgetManager::new_main_menu(),
            fall_speed_divide: FALL_SPEED_DIVIDE,
            freeze: FREEZE,
            clock: 0.0,
        }
    }

    pub fn handle_text_input(&mut self, input: &str) {
        self.widget_manager.handle_text_input(input)
    }

    pub fn handle_key_press(&mut self, key: Key) {
        if matches!(self.view_state, ViewState::Game(_)) {
            let game_flow_change = self.game.handle_key_press_in_game(key);
            self.game.update_game_flow(game_flow_change);
        }
        self.widget_manager.handle_key_press(key);
    }

    pub fn handle_remote(&mut self) {
        if let Some(view) = self.game.handle_remote() {
            self.set_view(view);
        }
    }

    pub fn handle_key_release(&mut self, key: Key) {
        if matches!(self.view_state, ViewState::Game(_)) {
            self.game.handle_key_release(key);
        }
    }

    pub fn is_remote(&self) -> bool {
        self.game.player_config.is_remote()
    }

    pub fn is_multiplayer(&self) -> bool {
        self.game.player_config.is_multiplayer()
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton) {
        if button == MouseButton::Left {
            self.widget_manager.handle_left_click(&self.cursor_position);
        }
    }

    pub fn handle_mouse_release(&mut self, button: MouseButton) {
        if button == MouseButton::Left {
            self.widget_manager.handle_left_click_release();
        }
    }

    pub(crate) fn set_view(&mut self, view_state: ViewState) {
        println!("setting view to {:?}", view_state);
        let from_game = matches!(self.view_state, ViewState::Game(_));
        if self.view_state == ViewState::Settings {
            for (id, player) in self.game.mut_local_players().iter_mut().enumerate() {
                player.set_new_keybindings(&self.widget_manager.get_new_keybindings(id));
            }
        }
        self.view_state = view_state;
        match self.view_state {
            ViewState::MainMenu => {
                self.widget_manager = InteractiveWidgetManager::new_main_menu();
            }
            ViewState::Settings => {
                self.widget_manager =
                    InteractiveWidgetManager::new_settings(self.game.keybindings(), from_game);
            }
            ViewState::Game(nb_players) => {
                self.widget_manager = InteractiveWidgetManager::new_tetris_game(nb_players);
            }
            ViewState::CreateRoom => {
                /* let mut file = File::create("local_port.txt").unwrap();
                file.write(HOST_PORT.as_bytes()).unwrap(); */
                let local_ip = local_ip().unwrap().to_string() + HOST_PORT;
                //let local_ip = "127.0.0.1".to_string() + HOST_PORT;
                self.game.set_player_config(PlayerConfig::Viewer(local_ip));
                self.widget_manager = InteractiveWidgetManager::new_create_room();
            }
            ViewState::JoinRoom => {
                /* let mut file = File::create("local_port.txt").unwrap();
                file.write(GUEST_PORT.as_bytes()).unwrap(); */
                self.widget_manager = InteractiveWidgetManager::new_join_room();
            }
        }
    }
}
