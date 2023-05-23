use piston::MouseButton;
use crate::settings::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use crate::ui::button::Button;

pub struct MainMenu {
    pub create_single_player_game_button: Button,
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
            )
        }
    }

    pub fn handle_mouse_press(&self, button: MouseButton) {
        todo!()
    }

    pub fn handle_mouse_release(&self, button: MouseButton) {
        todo!()
    }
}