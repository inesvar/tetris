use std::collections::HashMap;
use crate::settings::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use crate::ui::button::Button;
use piston::MouseButton;
use crate::ui::text_input::TextInput;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum ButtonType {
    CreateSinglePlayerGameButton,
    CreateRoomButton,
    JoinRoomButton,
    SettingsButton,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum TextInputType {
    DebugTextInput,
}

pub struct InteractiveWidgetManager {
    pub(in crate::ui) buttons: HashMap<ButtonType, Button>,
    pub(in crate::ui) text_inputs: HashMap<TextInputType, TextInput>,
}

impl InteractiveWidgetManager {
    pub fn new() -> InteractiveWidgetManager {
        let mut create_single_player_game_button = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0,
            300.0,
            50.0,
            "New single-player game",
        );

        let mut create_room_button = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 100.0,
            300.0,
            50.0,
            "Create new room",
        );

        let mut join_room_button = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 200.0,
            300.0,
            50.0,
            "Join room",
        );

        let mut settings_button = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 300.0,
            300.0,
            50.0,
            "Settings",
        );

        let mut buttons = HashMap::new();
        buttons.insert(
            ButtonType::CreateSinglePlayerGameButton,
            create_single_player_game_button,
        );
        buttons.insert(ButtonType::CreateRoomButton, create_room_button);
        buttons.insert(ButtonType::JoinRoomButton, join_room_button);
        buttons.insert(ButtonType::SettingsButton, settings_button);

        let mut text_inputs = HashMap::new();
        text_inputs.insert(TextInputType::DebugTextInput, TextInput::new(
            100.0,
            50.0,
            200.0,
            50.0,
            "Type here...",
        ));

        InteractiveWidgetManager {
            buttons,
            text_inputs,
        }
    }

    pub fn handle_mouse_press(&mut self, mouse_button: MouseButton, cursor_position: &[f64; 2]) {
        for button in self.buttons.values_mut() {
            button.handle_mouse_press(mouse_button, cursor_position);
        }

        for text_input in self.text_inputs.values_mut() {
            text_input.handle_mouse_press(mouse_button, cursor_position);
        }
    }

    pub fn handle_mouse_release(&mut self, mouse_button: MouseButton, cursor_position: &[f64; 2]) {
        for button in self.buttons.values_mut() {
            button.handle_mouse_release(mouse_button, cursor_position);
        }
    }

    pub fn handle_key_press(&mut self, key: piston::Key) {
        for text_input in self.text_inputs.values_mut() {
            text_input.handle_key_press(key);
        }
    }

    pub fn handle_text_input(&mut self, text: &String) {
        for text_input in self.text_inputs.values_mut() {
            text_input.handle_text_input(text);
        }
    }

    pub fn get_button(&self, button_type: ButtonType) -> &Button {
        self.buttons.get(&button_type).unwrap_or_else(|| panic!("Button {:?} not found", button_type))
    }

    pub fn get_input(&self, input_type: TextInputType) -> &TextInput {
        self.text_inputs.get(&input_type).unwrap_or_else(|| panic!("Input {:?} not found", input_type))
    }
}
