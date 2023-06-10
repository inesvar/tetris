use piston::MouseButton;
use crate::settings::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use crate::ui::button::Button;

pub struct MainMenu {
    pub create_single_player_game_button: Button,
    pub create_room_button: Button,
    pub join_room_button: Button,
    pub settings_button: Button,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            create_single_player_game_button: Button::new(
                DEFAULT_WINDOW_WIDTH as f64 / 2.0,
                DEFAULT_WINDOW_HEIGHT as f64 / 2.0,
                300.0,
                50.0,
                String::from("Create new single-player game"),
            ),
            create_room_button: Button::new(
                DEFAULT_WINDOW_WIDTH as f64 / 2.0,
                DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 100.0,
                300.0,
                50.0,
                String::from("Create new room"),
            ),
            join_room_button: Button::new(
                DEFAULT_WINDOW_WIDTH as f64 / 2.0,
                DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 200.0,
                300.0,
                50.0,
                String::from("Join room"),
            ),
            settings_button: Button::new(
                DEFAULT_WINDOW_WIDTH as f64 / 2.0,
                DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 300.0,
                300.0,
                50.0,
                String::from("Settings"),
            ),
        }
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton, cursor_position: &[f64; 2]) {
        self.create_single_player_game_button.handle_mouse_press(button, cursor_position);
        self.create_room_button.handle_mouse_press(button, cursor_position);
        self.join_room_button.handle_mouse_press(button, cursor_position);
        self.settings_button.handle_mouse_press(button, cursor_position);
    }

    pub fn handle_mouse_release(&mut self, button: MouseButton, cursor_position: &[f64; 2]) {
        self.create_single_player_game_button.handle_mouse_release(button, cursor_position);
        self.create_room_button.handle_mouse_release(button, cursor_position);
        self.join_room_button.handle_mouse_release(button, cursor_position);
        self.settings_button.handle_mouse_release(button, cursor_position);
    }
}