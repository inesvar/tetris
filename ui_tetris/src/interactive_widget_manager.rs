use super::{button::Button, key_input::KeyInput, text::Text, text_input::TextInput};
use super::{
    DEFAULT_BUTTON_HEIGHT, DEFAULT_BUTTON_WIDTH, DEFAULT_BUTTON_Y_SPACING, DEFAULT_FONT_SIZE,
    DEFAULT_KEY_INPUT_HEIGHT, DEFAULT_KEY_INPUT_WIDTH, DEFAULT_SCORE_TEXT_Y, DEFAULT_TITLE_Y,
    DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, GUEST_PORT, HOST_PORT, SILVER,
};
use crate::keybindings::Keybindings;
use arboard::Clipboard;
use core_tetris::TetrisCommand;
use graphics::math::Scalar;
use render_tetris::{BLOCK_SIZE, DEFAULT_GRID_X};
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
pub enum TextType {
    Title,
    Restart,
    Pause,
    Timer,
}

pub struct InteractiveWidgetManager {
    pub(super) buttons: HashMap<ButtonType, Button>,
    pub(super) text_inputs: HashMap<TextInputType, TextInput>,
    pub(super) key_inputs: HashMap<(TetrisCommand, usize), KeyInput>,
    pub(super) texts: HashMap<TextType, Text>,
}

impl InteractiveWidgetManager {
    pub fn new_main_menu() -> InteractiveWidgetManager {
        let create_single_player_game_button = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 - DEFAULT_BUTTON_Y_SPACING,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Single-player game",
        );

        let create_room_button = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Create new room",
        );

        let join_room_button = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Join room",
        );

        let settings_button = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 2.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Settings",
        );

        let create_two_player_game_button = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 3.0,
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

        let title_text = Text::new_with_tetris_font(
            "T",
            DEFAULT_FONT_SIZE,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );

        let mut texts = HashMap::new();
        texts.insert(TextType::Title, title_text);

        let text_inputs = HashMap::new();

        let key_inputs = HashMap::new();

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
            texts,
        }
    }

    pub fn new_one_player_settings(
        settings: &Keybindings,
        from_game: bool,
    ) -> InteractiveWidgetManager {
        let fall_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH / 4.0,
            DEFAULT_WINDOW_HEIGHT / 2.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            settings.get_keys(TetrisCommand::SoftDrop),
            "Soft Drop Keys :",
        );

        let hard_drop_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH / 4.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            settings.get_keys(TetrisCommand::HardDrop),
            "Hard Drop Keys :",
        );

        let right_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH / 4.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 2.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            settings.get_keys(TetrisCommand::Right),
            "Right Keys :",
        );

        let left_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH / 4.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 3.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            settings.get_keys(TetrisCommand::Left),
            "Left Keys :",
        );

        let rotate_clockwise_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH * 3.0 / 4.0,
            DEFAULT_WINDOW_HEIGHT / 2.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            settings.get_keys(TetrisCommand::Clockwise),
            "Rotate Clockwise Keys :",
        );

        let rotate_counterclockwise_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH * 3.0 / 4.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            settings.get_keys(TetrisCommand::Counterclockwise),
            "Rotate Counterclockwise Keys :",
        );

        let rotate_half_turn_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH * 3.0 / 4.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 2.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            settings.get_keys(TetrisCommand::HalfTurn),
            "Rotate Half Turn Keys :",
        );

        let hold_tetromino_keys_input = KeyInput::new_with_info(
            DEFAULT_WINDOW_WIDTH * 3.0 / 4.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 3.0,
            DEFAULT_KEY_INPUT_WIDTH,
            DEFAULT_KEY_INPUT_HEIGHT,
            settings.get_keys(TetrisCommand::Hold),
            "Hold Tetromino Keys :",
        );

        let mut buttons = HashMap::new();

        if !from_game {
            let back_to_main_menu_button = Button::new(
                (5.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
                (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
                DEFAULT_BUTTON_WIDTH / 6.0,
                DEFAULT_BUTTON_HEIGHT / 2.0,
                "Back",
            );
            buttons.insert(ButtonType::BackToMainMenu, back_to_main_menu_button);
        } else {
            let back_to_game_button = Button::new(
                (60.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
                (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
                DEFAULT_BUTTON_WIDTH / 6.0,
                DEFAULT_BUTTON_HEIGHT / 2.0,
                "Back",
            );
            buttons.insert(ButtonType::BackToGame, back_to_game_button);
        }

        let text_inputs = HashMap::new();

        let title_text = Text::new_with_tetris_font(
            "T",
            DEFAULT_FONT_SIZE,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );

        let mut texts = HashMap::new();
        texts.insert(TextType::Title, title_text);

        let mut key_inputs = HashMap::new();
        key_inputs.insert((TetrisCommand::SoftDrop, 0), fall_keys_input);
        key_inputs.insert((TetrisCommand::HardDrop, 0), hard_drop_keys_input);
        key_inputs.insert((TetrisCommand::Right, 0), right_keys_input);
        key_inputs.insert((TetrisCommand::Left, 0), left_keys_input);
        key_inputs.insert((TetrisCommand::Clockwise, 0), rotate_clockwise_keys_input);
        key_inputs.insert(
            (TetrisCommand::Counterclockwise, 0),
            rotate_counterclockwise_keys_input,
        );
        key_inputs.insert((TetrisCommand::HalfTurn, 0), rotate_half_turn_keys_input);
        key_inputs.insert((TetrisCommand::Hold, 0), hold_tetromino_keys_input);

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
            texts,
        }
    }

    pub fn new_two_players_settings(
        settings1: &Keybindings,
        settings2: &Keybindings,
        from_game: bool,
    ) -> InteractiveWidgetManager {
        let settings = [settings1, settings2];
        let player_x = [0.0, DEFAULT_WINDOW_WIDTH];

        let text_inputs = HashMap::new();
        let mut key_inputs = HashMap::new();

        for i in 0..=1 {
            let fall_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH / 4.0 + player_x[i],
                DEFAULT_WINDOW_HEIGHT / 2.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                settings[i].get_keys(TetrisCommand::SoftDrop),
                "Soft Drop Keys :",
            );

            let hard_drop_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH / 4.0 + player_x[i],
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                settings[i].get_keys(TetrisCommand::HardDrop),
                "Hard Drop Keys :",
            );

            let right_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH / 4.0 + player_x[i],
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 2.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                settings[i].get_keys(TetrisCommand::Right),
                "Right Keys :",
            );

            let left_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH / 4.0 + player_x[i],
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 3.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                settings[i].get_keys(TetrisCommand::Left),
                "Left Keys :",
            );

            let rotate_clockwise_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH * 3.0 / 4.0 + player_x[i],
                DEFAULT_WINDOW_HEIGHT / 2.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                settings[i].get_keys(TetrisCommand::Clockwise),
                "Rotate Clockwise Keys :",
            );

            let rotate_counterclockwise_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH * 3.0 / 4.0 + player_x[i],
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                settings[i].get_keys(TetrisCommand::Counterclockwise),
                "Rotate Counterclockwise Keys :",
            );

            let rotate_half_turn_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH * 3.0 / 4.0 + player_x[i],
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 2.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                settings[i].get_keys(TetrisCommand::HalfTurn),
                "Rotate Half Turn Keys :",
            );

            let hold_tetromino_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH * 3.0 / 4.0 + player_x[i],
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 3.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                settings[i].get_keys(TetrisCommand::Hold),
                "Hold Tetromino Keys :",
            );

            key_inputs.insert((TetrisCommand::SoftDrop, i), fall_keys_input);
            key_inputs.insert((TetrisCommand::HardDrop, i), hard_drop_keys_input);
            key_inputs.insert((TetrisCommand::Right, i), right_keys_input);
            key_inputs.insert((TetrisCommand::Left, i), left_keys_input);
            key_inputs.insert((TetrisCommand::Clockwise, i), rotate_clockwise_keys_input);
            key_inputs.insert(
                (TetrisCommand::Counterclockwise, i),
                rotate_counterclockwise_keys_input,
            );
            key_inputs.insert((TetrisCommand::HalfTurn, i), rotate_half_turn_keys_input);
            key_inputs.insert((TetrisCommand::Hold, i), hold_tetromino_keys_input);
        }

        let mut buttons = HashMap::new();
        if !from_game {
            let back_to_main_menu_button = Button::new(
                (5.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
                (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
                DEFAULT_BUTTON_WIDTH / 6.0,
                DEFAULT_BUTTON_HEIGHT / 2.0,
                "Back",
            );
            buttons.insert(ButtonType::BackToMainMenu, back_to_main_menu_button);
        } else {
            let back_to_game_button = Button::new(
                (125.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
                (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
                DEFAULT_BUTTON_WIDTH / 6.0,
                DEFAULT_BUTTON_HEIGHT / 2.0,
                "Back",
            );
            buttons.insert(ButtonType::BackToGame, back_to_game_button);
        }

        let title_text = Text::new_with_tetris_font(
            "T",
            DEFAULT_FONT_SIZE,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );

        let mut texts = HashMap::new();
        texts.insert(TextType::Title, title_text);

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
            texts,
        }
    }

    pub fn new_single_player_game() -> InteractiveWidgetManager {
        let back_to_main_menu_button = Button::new(
            (5.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let pause_button = Button::new(
            (13.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 5.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Pause",
        );

        let settings_button = Button::new(
            (58.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 3.5,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Settings",
        );

        let mut buttons = HashMap::new();
        buttons.insert(ButtonType::BackToMainMenu, back_to_main_menu_button);
        buttons.insert(ButtonType::ToPause, pause_button);
        buttons.insert(ButtonType::ToSettings, settings_button);

        let title_text = Text::new_with_tetris_font(
            "T",
            DEFAULT_FONT_SIZE,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );
        let restart_text = Text::new(
            "Press R to (re)start",
            (DEFAULT_FONT_SIZE * 22) / 16,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );
        let timer_text = Text::new(
            "",
            DEFAULT_FONT_SIZE,
            DEFAULT_GRID_X - 4.0 * BLOCK_SIZE,
            DEFAULT_SCORE_TEXT_Y + 1.5 * BLOCK_SIZE,
            SILVER,
        );
        let pause_text = Text::new(
            "Press P to resume",
            (DEFAULT_FONT_SIZE * 22) / 16,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );

        let mut texts = HashMap::new();
        texts.insert(TextType::Title, title_text);
        texts.insert(TextType::Restart, restart_text);
        texts.insert(TextType::Timer, timer_text);
        texts.insert(TextType::Pause, pause_text);

        let text_inputs = HashMap::new();
        let key_inputs = HashMap::new();

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
            texts,
        }
    }

    pub fn new_two_player_game() -> InteractiveWidgetManager {
        let back_to_main_menu_button = Button::new(
            (5.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let pause_button = Button::new(
            (13.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 5.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Pause",
        );

        let settings_button = Button::new(
            (123.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 3.5,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Settings",
        );

        let mut buttons = HashMap::new();
        buttons.insert(ButtonType::BackToMainMenu, back_to_main_menu_button);
        buttons.insert(ButtonType::ToPause, pause_button);
        buttons.insert(ButtonType::ToSettings, settings_button);

        let title_text = Text::new_with_tetris_font(
            "T",
            DEFAULT_FONT_SIZE,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );
        let restart_text = Text::new(
            "Press R to (re)start",
            (DEFAULT_FONT_SIZE * 22) / 16,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );
        let timer_text = Text::new(
            "",
            DEFAULT_FONT_SIZE,
            DEFAULT_GRID_X - 4.0 * BLOCK_SIZE,
            DEFAULT_SCORE_TEXT_Y + 1.5 * BLOCK_SIZE,
            SILVER,
        );
        let pause_text = Text::new(
            "Press P to resume",
            (DEFAULT_FONT_SIZE * 22) / 16,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );

        let mut texts = HashMap::new();
        texts.insert(TextType::Title, title_text);
        texts.insert(TextType::Restart, restart_text);
        texts.insert(TextType::Timer, timer_text);
        texts.insert(TextType::Pause, pause_text);

        let text_inputs = HashMap::new();
        let key_inputs = HashMap::new();

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
            texts,
        }
    }

    pub fn new_create_room() -> InteractiveWidgetManager {
        let back_to_main_menu_button = Button::new(
            (5.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let copy_ip = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Copy room IP",
        );

        let mut buttons = HashMap::new();
        buttons.insert(ButtonType::BackToMainMenu, back_to_main_menu_button);
        buttons.insert(ButtonType::CopyToClipboard, copy_ip);

        let title_text = Text::new_with_tetris_font(
            "T",
            DEFAULT_FONT_SIZE,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );

        let mut texts = HashMap::new();
        texts.insert(TextType::Title, title_text);

        let text_inputs = HashMap::new();
        let key_inputs = HashMap::new();

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
            texts,
        }
    }

    pub fn new_join_room() -> InteractiveWidgetManager {
        let back_to_main_menu_button = Button::new(
            (5.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let join_room = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Join room",
        );

        let room_ip_input = TextInput::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 3.0,
            DEFAULT_BUTTON_WIDTH,
            DEFAULT_BUTTON_HEIGHT,
            "Join room",
        );

        let paste_ip = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
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

        let title_text = Text::new_with_tetris_font(
            "T",
            DEFAULT_FONT_SIZE,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );

        let mut texts = HashMap::new();
        texts.insert(TextType::Title, title_text);
        let key_inputs = HashMap::new();

        InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
            texts,
        }
    }

    pub fn handle_left_click(&mut self, cursor_position: &[Scalar; 2]) {
        for text_input in self.text_inputs.values_mut() {
            text_input.handle_left_click(cursor_position);
        }
        for key_input in self.key_inputs.values_mut() {
            key_input.handle_left_click(cursor_position);
        }

        for button in self.buttons.values_mut() {
            button.handle_left_click(cursor_position);
        }
    }

    pub fn handle_left_click_release(&mut self) {
        for button in self.buttons.values_mut() {
            button.handle_left_click_release();
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

    pub fn get_new_keybindings(&mut self, player: usize) -> Keybindings {
        let mut keybindings_manager = Keybindings::default();
        for ((command, id), key_input) in self.key_inputs.iter_mut() {
            if *id != player {
                continue;
            }
            if key_input.keys.is_empty() {
                keybindings_manager.set_keys(*command, key_input.initial_keys.clone());
            } else {
                keybindings_manager.set_keys(*command, key_input.keys.clone());
            }
        }
        keybindings_manager
    }

    pub fn set_text(&mut self, text_type: TextType, new_text: String) {
        if let Some(text) = self.texts.get_mut(&text_type) {
            text.set_text(new_text);
        }
    }

    pub fn update_clipboard(&mut self) {
        if let Some(button) = self.buttons.get_mut(&ButtonType::CopyToClipboard) {
            if button.has_been_pressed() {
                let ip = "127.0.0.1".to_string();
                let text = format!("{}{}", ip, HOST_PORT);
                let mut clipboard = Clipboard::new().expect("Clipboard is not supported");
                clipboard
                    .set_text(text)
                    .expect("Setting the clipboard failed");
            }
        };
        if let Some(button) = self.buttons.get_mut(&ButtonType::PasteFromClipboard) {
            if button.has_been_pressed() {
                let mut clipboard = Clipboard::new().expect("Clipboard is not supported");
                let ip = clipboard.get_text().expect("Getting the clipboard failed");
                let text_input = self.get_input(TextInputType::IpAddressInput);
                text_input.text.set_text(ip);
            }
        };
    }

    pub fn update_from_text(&mut self) {
        let button = self.get_button(&ButtonType::ToTwoRemoteGame);
        if button.has_been_pressed() {
            let text_input = self.get_input(TextInputType::IpAddressInput);
            let remote_ip = String::from(text_input.text.get_text());
            println!("remote ip is {remote_ip}");
            let local_ip = "127.0.0.1".to_string();
            let local_ip = format!("{}{}", local_ip, GUEST_PORT);

            let join_room = Button::new_pressed(
                DEFAULT_WINDOW_WIDTH / 2.0,
                DEFAULT_WINDOW_HEIGHT / 2.0,
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
            if button_type.view_changer() && button.has_been_pressed() {
                println!("button type is {:?}", button_type);
                return button_type.clone();
            }
        }
        ButtonType::Nothing
    }
}
