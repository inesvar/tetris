use crate::settings::{
    Keybindings, DEFAULT_BUTTON_HEIGHT, DEFAULT_BUTTON_WIDTH, DEFAULT_BUTTON_Y_SPACING,
    DEFAULT_KEY_INPUT_HEIGHT, DEFAULT_KEY_INPUT_WIDTH, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH,
    GUEST_PORT, HOST_PORT,
};
use crate::ui::{button::Button, key_input::KeyInput, text_input::TextInput};
use local_ip_address::local_ip;
use piston::{Key, MouseButton};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum ButtonType {
    ToSinglePlayerGame,
    ToTwoRemoteGame,
    ToTwoRemoteGameInfo { local_ip: String, remote_ip: String },
    ToCreateRoom,
    ToJoinRoom,
    ToSettings,
    BackToMainMenu,
    BackToGame,
    ToTwoLocalGame,
    ToPause,
    CopyToClipboard,
    PasteFromClipboard,
    Nothing,
}

impl ButtonType {
    fn view_changer(&self) -> bool {
        *self != Self::CopyToClipboard
            && *self != Self::ToTwoRemoteGame
            && *self != Self::PasteFromClipboard
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum TextInputType {
    #[allow(unused)]
    DebugTextInput,
    IpAddressInput,
}

#[derive(Hash, PartialEq, Eq)]
pub enum TetrisCommand {
    Fall(Vec<Key>),
    HardDrop(Vec<Key>),
    Right(Vec<Key>),
    Left(Vec<Key>),
    RotateClockwise(Vec<Key>),
    RotateCounterclockwise(Vec<Key>),
    HoldTetromino(Vec<Key>),
}

#[allow(clippy::enum_variant_names)]
#[derive(PartialEq)]
pub enum SettingsType {
    OnePlayer,
    LeftPlayer,
    RightPlayer,
}

pub struct InteractiveWidgetManager {
    pub(super) buttons: HashMap<ButtonType, Button>,
    pub(super) text_inputs: HashMap<TextInputType, TextInput>,
    pub(super) key_inputs: HashMap<TetrisCommand, KeyInput>,
}

impl InteractiveWidgetManager {
    pub fn new_main_menu() -> InteractiveWidgetManager {
        let create_single_player_game_button = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 - DEFAULT_BUTTON_Y_SPACING,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Single-player game",
        );

        let create_room_button = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Create new room",
        );

        let join_room_button = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Join room",
        );

        let settings_button = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + DEFAULT_BUTTON_Y_SPACING * 2.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Settings",
        );

        let create_two_player_game_button = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + DEFAULT_BUTTON_Y_SPACING * 3.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "two-player game",
        );

        let mut buttons = HashMap::new();
        buttons.insert(
            ButtonType::ToSinglePlayerGame,
            create_single_player_game_button,
        );
        buttons.insert(ButtonType::ToCreateRoom, create_room_button);
        buttons.insert(ButtonType::ToJoinRoom, join_room_button);
        buttons.insert(ButtonType::ToSettings, settings_button);
        buttons.insert(ButtonType::ToTwoLocalGame, create_two_player_game_button);

        let text_inputs = HashMap::new();

        let key_inputs = HashMap::new();

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
        }
    }

    pub fn new_settings(
        settings: &Keybindings,
        settings_type: SettingsType,
        from_game: bool,
    ) -> InteractiveWidgetManager {
        let player_x = if settings_type == SettingsType::RightPlayer {
            DEFAULT_WINDOW_WIDTH as f64
        } else {
            0.0
        };
        let fall_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 / 4.0 + player_x,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.fall_keys,
            "Fall Keys :",
        );

        let hard_drop_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 / 4.0 + player_x,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.hard_drop_keys,
            "Hard Drop Keys :",
        );

        let right_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 / 4.0 + player_x,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + DEFAULT_BUTTON_Y_SPACING * 2.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.right_keys,
            "Right Keys :",
        );

        let left_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 / 4.0 + player_x,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + DEFAULT_BUTTON_Y_SPACING * 3.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.left_keys,
            "Left Keys :",
        );

        let rotate_clockwise_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 * 3.0 / 4.0 + player_x,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.rotate_clockwise_keys,
            "Rotate Clockwise Keys :",
        );

        let rotate_counterclockwise_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 * 3.0 / 4.0 + player_x,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.rotate_counterclockwise_keys,
            "Rotate Counterclockwise Keys :",
        );

        let hold_tetromino_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH as f64 * 3.0 / 4.0 + player_x,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + DEFAULT_BUTTON_Y_SPACING * 2.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            &settings.hold_tetromino_keys,
            "Hold Tetromino Keys :",
        );

        let mut buttons = HashMap::new();

        if !from_game {
            match settings_type {
                SettingsType::OnePlayer => {
                    let back_to_main_menu_button = Button::new(
                        (5.0 * DEFAULT_WINDOW_WIDTH as f64) / 65.0,
                        (5.0 * DEFAULT_WINDOW_HEIGHT as f64) / 70.0,
                        DEFAULT_BUTTON_WIDTH / 6.0,
                        DEFAULT_BUTTON_HEIGHT / 2.0,
                        "Back",
                    );
                    buttons.insert(ButtonType::BackToMainMenu, back_to_main_menu_button);
                }
                SettingsType::RightPlayer => {}
                SettingsType::LeftPlayer => {
                    let back_to_main_menu_button = Button::new(
                        (5.0 * DEFAULT_WINDOW_WIDTH as f64) / 65.0,
                        (5.0 * DEFAULT_WINDOW_HEIGHT as f64) / 70.0,
                        DEFAULT_BUTTON_WIDTH / 6.0,
                        DEFAULT_BUTTON_HEIGHT / 2.0,
                        "Back",
                    );
                    buttons.insert(ButtonType::BackToMainMenu, back_to_main_menu_button);
                }
            }
        } else {
            match settings_type {
                SettingsType::OnePlayer => {
                    let back_to_game_button = Button::new(
                        (60.0 * DEFAULT_WINDOW_WIDTH as f64) / 65.0,
                        (5.0 * DEFAULT_WINDOW_HEIGHT as f64) / 70.0,
                        DEFAULT_BUTTON_WIDTH / 6.0,
                        DEFAULT_BUTTON_HEIGHT / 2.0,
                        "Back",
                    );
                    buttons.insert(ButtonType::BackToGame, back_to_game_button);
                }
                SettingsType::RightPlayer => {}
                SettingsType::LeftPlayer => {
                    let back_to_game_button = Button::new(
                        (125.0 * DEFAULT_WINDOW_WIDTH as f64) / 65.0,
                        (5.0 * DEFAULT_WINDOW_HEIGHT as f64) / 70.0,
                        DEFAULT_BUTTON_WIDTH / 6.0,
                        DEFAULT_BUTTON_HEIGHT / 2.0,
                        "Back",
                    );
                    buttons.insert(ButtonType::BackToGame, back_to_game_button);
                }
            }
        }

        let text_inputs = HashMap::new();

        let mut key_inputs = HashMap::new();
        key_inputs.insert(
            TetrisCommand::Fall(fall_keys_input.keys.clone()),
            fall_keys_input,
        );
        key_inputs.insert(
            TetrisCommand::HardDrop(hard_drop_keys_input.keys.clone()),
            hard_drop_keys_input,
        );
        key_inputs.insert(
            TetrisCommand::Right(right_keys_input.keys.clone()),
            right_keys_input,
        );
        key_inputs.insert(
            TetrisCommand::Left(left_keys_input.keys.clone()),
            left_keys_input,
        );
        key_inputs.insert(
            TetrisCommand::RotateClockwise(rotate_clockwise_keys_input.keys.clone()),
            rotate_clockwise_keys_input,
        );
        key_inputs.insert(
            TetrisCommand::RotateCounterclockwise(rotate_counterclockwise_keys_input.keys.clone()),
            rotate_counterclockwise_keys_input,
        );
        key_inputs.insert(
            TetrisCommand::HoldTetromino(hold_tetromino_keys_input.keys.clone()),
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
            (5.0 * DEFAULT_WINDOW_WIDTH as f64) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT as f64) / 70.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let pause_button = Button::new(
            (13.0 * DEFAULT_WINDOW_WIDTH as f64) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT as f64) / 70.0,
            DEFAULT_BUTTON_WIDTH / 5.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Pause",
        );

        let settings_button = Button::new(
            (58.0 * DEFAULT_WINDOW_WIDTH as f64) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT as f64) / 70.0,
            DEFAULT_BUTTON_WIDTH / 3.5,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Settings",
        );

        let mut buttons = HashMap::new();
        buttons.insert(ButtonType::BackToMainMenu, back_to_main_menu_button);
        buttons.insert(ButtonType::ToPause, pause_button);
        buttons.insert(ButtonType::ToSettings, settings_button);

        let text_inputs = HashMap::new();
        let key_inputs = HashMap::new();

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
        }
    }

    pub fn new_two_player_game() -> InteractiveWidgetManager {
        let back_to_main_menu_button = Button::new(
            (5.0 * DEFAULT_WINDOW_WIDTH as f64) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT as f64) / 70.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let pause_button = Button::new(
            (13.0 * DEFAULT_WINDOW_WIDTH as f64) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT as f64) / 70.0,
            DEFAULT_BUTTON_WIDTH / 5.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Pause",
        );

        let settings_button = Button::new(
            (123.0 * DEFAULT_WINDOW_WIDTH as f64) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT as f64) / 70.0,
            DEFAULT_BUTTON_WIDTH / 3.5,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Settings",
        );

        let mut buttons = HashMap::new();
        buttons.insert(ButtonType::BackToMainMenu, back_to_main_menu_button);
        buttons.insert(ButtonType::ToPause, pause_button);
        buttons.insert(ButtonType::ToSettings, settings_button);

        let text_inputs = HashMap::new();
        let key_inputs = HashMap::new();

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
        }
    }

    pub fn new_create_room() -> InteractiveWidgetManager {
        let back_to_main_menu_button = Button::new(
            (5.0 * DEFAULT_WINDOW_WIDTH as f64) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT as f64) / 70.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let copy_ip = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Copy room IP",
        );

        let mut buttons = HashMap::new();
        buttons.insert(ButtonType::BackToMainMenu, back_to_main_menu_button);
        buttons.insert(ButtonType::CopyToClipboard, copy_ip);
        let text_inputs = HashMap::new();
        let key_inputs = HashMap::new();

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
        }
    }

    pub fn new_join_room() -> InteractiveWidgetManager {
        let back_to_main_menu_button = Button::new(
            (5.0 * DEFAULT_WINDOW_WIDTH as f64) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT as f64) / 70.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let join_room = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Join room",
        );

        let room_ip_input = TextInput::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 3.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Join room",
        );

        let paste_ip = Button::new(
            DEFAULT_WINDOW_WIDTH as f64 / 2.0,
            DEFAULT_WINDOW_HEIGHT as f64 / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Paste",
        );

        let mut buttons = HashMap::new();
        buttons.insert(ButtonType::BackToMainMenu, back_to_main_menu_button);
        buttons.insert(ButtonType::ToTwoRemoteGame, join_room);
        buttons.insert(ButtonType::PasteFromClipboard, paste_ip);
        let mut text_inputs = HashMap::new();
        text_inputs.insert(TextInputType::IpAddressInput, room_ip_input);
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

    pub fn handle_text_input(&mut self, text: &str) {
        for text_input in self.text_inputs.values_mut() {
            text_input.handle_text_input(text);
        }
    }

    pub fn get_button(&mut self, button_type: &ButtonType) -> &mut Button {
        self.buttons
            .get_mut(button_type)
            .unwrap_or_else(|| panic!("Button {:?} not found", button_type))
    }

    pub fn get_input(&mut self, input_type: TextInputType) -> &mut TextInput {
        self.text_inputs
            .get_mut(&input_type)
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

    #[allow(warnings)]
    pub fn update_clipboard(&mut self) {
        todo!();
        if let Some(button) = self.buttons.get_mut(&ButtonType::CopyToClipboard) {
            if button.commit() {
                println!("supposed to COPY");
                let ip = local_ip().unwrap().to_string();
                //let ip = "127.0.0.1".to_string();
                let text = format!("{}{}", ip, HOST_PORT);
                // TODO set clipboard to text
                // let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                // ctx.set_contents(text.to_owned()).unwrap();
            }
        };
        if let Some(button) = self.buttons.get_mut(&ButtonType::PasteFromClipboard) {
            if button.commit() {
                println!("supposed to PASTE");
                // TODO get clipboard contents
                // let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                // let ip = ctx.get_contents().unwrap();
                let ip = String::from("");
                let text_input = self.get_input(TextInputType::IpAddressInput);
                text_input.text.content = ip;
            }
        };
    }

    pub fn update_from_text(&mut self) {
        let button = self.get_button(&ButtonType::ToTwoRemoteGame);
        if button.commit() {
            let text_input = self.get_input(TextInputType::IpAddressInput);
            let remote_ip = text_input.text.content.clone();
            println!("remote ip is {remote_ip}");
            let local_ip = local_ip().unwrap().to_string();
            //let local_ip = "127.0.0.1".to_string();
            let local_ip = format!("{}{}", local_ip, GUEST_PORT);

            let join_room = Button::new_committed(
                DEFAULT_WINDOW_WIDTH as f64 / 2.0,
                DEFAULT_WINDOW_HEIGHT as f64 / 2.0,
                DEFAULT_BUTTON_WIDTH,
                DEFAULT_BUTTON_HEIGHT,
                "Join this room",
            );

            self.buttons.insert(
                ButtonType::ToTwoRemoteGameInfo {
                    local_ip,
                    remote_ip,
                },
                join_room,
            );
        }
    }

    pub fn update_view(&mut self) -> ButtonType {
        for (button_type, button) in self.buttons.iter_mut() {
            if button_type.view_changer() && button.commit() {
                println!("button type is {:?}", button_type);
                return button_type.clone();
            }
        }
        ButtonType::Nothing
    }
}
