use crate::settings::{
    DEFAULT_BUTTON_HEIGHT, DEFAULT_BUTTON_WIDTH, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH,
};
use crate::ui::button::Button;
use crate::ui::text_input::TextInput;
use piston::{MouseButton, Key};
use std::collections::HashMap;
use crate::ui::key_input::KeyInput;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum ButtonType {
    NewSinglePlayerGame,
    CreateRoom,
    JoinRoom,
    Settings,
    BackToMainMenu,
    Pause,
    Nothing,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum TextInputType {
    DebugTextInput,
    IpAddressInput
}

#[derive(Hash, PartialEq, Eq)]
pub enum KeyInputType {
    FallKey(Vec<Key>),
    HardDropKey(Vec<Key>),
}

pub struct InteractiveWidgetManager {
    pub(in crate::ui) buttons: HashMap<ButtonType, Button>,
    pub(in crate::ui) text_inputs: HashMap<TextInputType, TextInput>,
    pub(in crate::ui) key_inputs: HashMap<KeyInputType, KeyInput>,
}

impl InteractiveWidgetManager {
    pub fn new_main_menu() -> InteractiveWidgetManager {
        let create_single_player_game_button = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "New single-player game",
        );

        let create_room_button = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 100.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Create new room",
        );

        let join_room_button = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 200.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Join room",
        );

        let settings_button = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 300.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Settings",
        );

        let mut buttons = HashMap::new();
        buttons.insert(
            ButtonType::NewSinglePlayerGame,
            create_single_player_game_button,
        );
        buttons.insert(ButtonType::CreateRoom, create_room_button);
        buttons.insert(ButtonType::JoinRoom, join_room_button);
        buttons.insert(ButtonType::Settings, settings_button);

        let mut text_inputs = HashMap::new();
        text_inputs.insert(
            TextInputType::DebugTextInput,
            TextInput::new_with_info(100.0, 100.0, 200.0, 50.0, "Type here...", "try this pls"),
        );

        let key_inputs = HashMap::new();

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
        }
    }

    pub fn new_settings() -> InteractiveWidgetManager {
        let fall_keys_text_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "S, Down",
            "Fall Keys :"
        );

        let hard_drop_text_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 100.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Space",
            "Hard Drop Keys :",
        );

        let back_to_main_menu_button = Button::new(
            50.0,
            50.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let mut buttons = HashMap::new();
        buttons.insert(ButtonType::BackToMainMenu, back_to_main_menu_button);
        let text_inputs = HashMap::new();

        let mut key_inputs = HashMap::new();
        key_inputs.insert(
            KeyInputType::FallKey(vec![Key::S, Key::Down]),
            fall_keys_text_input,
        );
        key_inputs.insert(
            KeyInputType::HardDropKey(vec![Key::S, Key::Down]),
            hard_drop_text_input,
        );

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
        }
    }

    pub fn new_single_player_game() -> InteractiveWidgetManager {
        let back_to_main_menu_button = Button::new(
            50.0,
            50.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let pause_button = Button::new(
            150.0,
            50.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Pause",
        );

        let mut buttons = HashMap::new();
        buttons.insert(ButtonType::BackToMainMenu, back_to_main_menu_button);
        buttons.insert(ButtonType::Pause, pause_button);

        let text_inputs = HashMap::new();
        let key_inputs = HashMap::new();

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
        }
    }

    pub fn new_empty() -> InteractiveWidgetManager {
        let buttons = HashMap::new();
        let text_inputs = HashMap::new();
        let key_inputs = HashMap::new();

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
        }
    }

    pub fn handle_mouse_press(
        &mut self,
        mouse_button: MouseButton,
        cursor_position: &[f64; 2],
    ) -> ButtonType {
        for text_input in self.text_inputs.values_mut() {
            text_input.handle_mouse_press(mouse_button, cursor_position);
        }
        for key_input in self.key_inputs.values_mut() {
            key_input.handle_mouse_press(mouse_button, cursor_position);
        }

        for (button_type, button) in self.buttons.iter_mut() {
            button.handle_mouse_press(mouse_button, cursor_position);
            if button.is_pressed() {
                println!("a button was pressed");
                return button_type.clone();
            }
        }
        ButtonType::Nothing
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
        for key_input in self.key_inputs.values_mut() {
            key_input.handle_key_press(key);
        }
    }

    pub fn handle_text_input(&mut self, text: &String) {
        for text_input in self.text_inputs.values_mut() {
            text_input.handle_text_input(text);
        }
        for key_input in self.key_inputs.values_mut() {
            key_input.handle_text_input(text);
        }
    }

    pub fn get_button(&self, button_type: ButtonType) -> &Button {
        self.buttons
            .get(&button_type)
            .unwrap_or_else(|| panic!("Button {:?} not found", button_type))
    }

    pub fn get_input(&self, input_type: TextInputType) -> &TextInput {
        self.text_inputs
            .get(&input_type)
            .unwrap_or_else(|| panic!("Input {:?} not found", input_type))
    }
}
