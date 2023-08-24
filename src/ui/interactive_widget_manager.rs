use crate::settings::{
    Keybindings, DEFAULT_BUTTON_HEIGHT, DEFAULT_BUTTON_WIDTH, DEFAULT_KEY_INPUT_HEIGHT,
    DEFAULT_KEY_INPUT_WIDTH, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH,
};
use crate::ui::button::Button;
use crate::ui::key_input::KeyInput;
use crate::ui::text_input::TextInput;
use piston::{Key, MouseButton};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum ButtonType {
    ToSinglePlayerGame,
    ToCreateRoom,
    ToJoinRoom,
    ToSettings,
    BackToMainMenu,
    ToPause,
    Nothing,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum TextInputType {
    DebugTextInput,
    IpAddressInput,
}

#[derive(Hash, PartialEq, Eq)]
pub enum KeyInputType {
    FallKey(Vec<Key>),
    HardDropKey(Vec<Key>),
    RightKey(Vec<Key>),
    LeftKey(Vec<Key>),
    RotateClockwiseKey(Vec<Key>),
    RotateCounterclockwiseKey(Vec<Key>),
    HoldTetrominoKey(Vec<Key>),
}

pub struct InteractiveWidgetManager {
    pub(super) buttons: HashMap<ButtonType, Button>,
    pub(super) text_inputs: HashMap<TextInputType, TextInput>,
    pub(super) key_inputs: HashMap<KeyInputType, KeyInput>,
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
            ButtonType::ToSinglePlayerGame,
            create_single_player_game_button,
        );
        buttons.insert(ButtonType::ToCreateRoom, create_room_button);
        buttons.insert(ButtonType::ToJoinRoom, join_room_button);
        buttons.insert(ButtonType::ToSettings, settings_button);

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

    pub fn new_settings(settings: &Keybindings) -> InteractiveWidgetManager {
        let fall_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 / 4.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.fall_keys,
            "Fall Keys :",
        );

        let hard_drop_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 / 4.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 100.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.hard_drop_keys,
            "Hard Drop Keys :",
        );

        let right_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 / 4.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 200.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.right_keys,
            "Right Keys :",
        );

        let left_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 / 4.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 300.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.left_keys,
            "Left Keys :",
        );

        let rotate_clockwise_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 * 3.0 / 4.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.rotate_clockwise_keys,
            "Rotate Clockwise Keys :",
        );

        let rotate_counterclockwise_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 * 3.0 / 4.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 100.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.rotate_counterclockwise_keys,
            "Rotate Counterclockwise Keys :",
        );

        let hold_tetromino_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 * 3.0 / 4.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + 200.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.hold_tetromino_keys,
            "Hold Tetromino Keys :",
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
            KeyInputType::FallKey(fall_keys_input.keys.clone()),
            fall_keys_input,
        );
        key_inputs.insert(
            KeyInputType::HardDropKey(hard_drop_keys_input.keys.clone()),
            hard_drop_keys_input,
        );
        key_inputs.insert(
            KeyInputType::RightKey(right_keys_input.keys.clone()),
            right_keys_input,
        );
        key_inputs.insert(
            KeyInputType::LeftKey(left_keys_input.keys.clone()),
            left_keys_input,
        );
        key_inputs.insert(
            KeyInputType::RotateClockwiseKey(rotate_clockwise_keys_input.keys.clone()),
            rotate_clockwise_keys_input,
        );
        key_inputs.insert(
            KeyInputType::RotateCounterclockwiseKey(
                rotate_counterclockwise_keys_input.keys.clone(),
            ),
            rotate_counterclockwise_keys_input,
        );
        key_inputs.insert(
            KeyInputType::HoldTetrominoKey(hold_tetromino_keys_input.keys.clone()),
            hold_tetromino_keys_input,
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
        buttons.insert(ButtonType::ToPause, pause_button);

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

    pub fn handle_mouse_press(&mut self, mouse_button: MouseButton, cursor_position: &[f64; 2]) {
        for text_input in self.text_inputs.values_mut() {
            text_input.handle_mouse_press(mouse_button, cursor_position);
        }
        for key_input in self.key_inputs.values_mut() {
            key_input.handle_mouse_press(mouse_button, cursor_position);
        }

        for button in self.buttons.values_mut() {
            button.handle_mouse_press(mouse_button, cursor_position);
        }
    }

    pub fn handle_mouse_release(&mut self, mouse_button: MouseButton) {
        for button in self.buttons.values_mut() {
            button.handle_mouse_release(mouse_button);
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

    pub fn update_settings(&mut self, keybindings_manager: &mut Keybindings) {
        for (key_type, key_input) in self.key_inputs.iter_mut() {
            if key_input.commit() {
                keybindings_manager.set_keys(key_type, key_input.keys.clone());
                keybindings_manager.print();
            }
        }
    }

    pub fn update_view(&mut self) -> ButtonType {
        for (button_type, button) in self.buttons.iter_mut() {
            if button.commit() {
                return button_type.clone();
            }
        }
        ButtonType::Nothing
    }
}
