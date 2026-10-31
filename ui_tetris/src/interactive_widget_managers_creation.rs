use super::interactive_widget_manager::{
    CurrentViewButton, InteractiveWidgetManager, TextInputType, TextType, ViewChangeButton,
};
use super::{button::Button, key_input::KeyInput, text::Text, text_input::TextInput};
use super::{
    DEFAULT_BUTTON_HEIGHT, DEFAULT_BUTTON_WIDTH, DEFAULT_BUTTON_Y_SPACING, DEFAULT_FONT_SIZE,
    DEFAULT_KEY_INPUT_HEIGHT, DEFAULT_KEY_INPUT_WIDTH, DEFAULT_SCORE_TEXT_Y, DEFAULT_TITLE_Y,
    DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, SILVER,
};
use crate::keybindings::Keybindings;
use core_tetris::TetrisCommand;
use render_tetris::{BLOCK_SIZE, DEFAULT_GRID_X};
use std::collections::HashMap;

impl InteractiveWidgetManager {
    pub fn new_main_menu() -> InteractiveWidgetManager {
        let create_single_player_game_button = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 - DEFAULT_BUTTON_Y_SPACING,
            "Single-player game",
        );

        let create_room_button = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0,
            "Create new room",
        );

        let join_room_button = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
            "Join room",
        );

        let settings_button = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 2.0,
            "Settings",
        );

        let create_two_player_game_button = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 3.0,
            "two-player game",
        );

        let mut buttons = HashMap::new();
        buttons.insert(
            ViewChangeButton::ToSinglePlayerGame.into(),
            create_single_player_game_button,
        );
        buttons.insert(ViewChangeButton::ToCreateRoom.into(), create_room_button);
        buttons.insert(ViewChangeButton::ToJoinRoom.into(), join_room_button);
        buttons.insert(ViewChangeButton::ToSettings.into(), settings_button);
        buttons.insert(
            ViewChangeButton::ToTwoLocalGame.into(),
            create_two_player_game_button,
        );

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

    pub fn new_settings(
        settings: impl Iterator<Item = Keybindings>,
        from_game: bool,
    ) -> InteractiveWidgetManager {
        let text_inputs = HashMap::new();
        let mut key_inputs = HashMap::new();
        let mut horizontal_offset = 0.0;

        for (i, setting) in settings.enumerate() {
            let fall_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                setting.get_keys(TetrisCommand::SoftDrop),
                "Soft Drop Keys :",
            );

            let hard_drop_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                setting.get_keys(TetrisCommand::HardDrop),
                "Hard Drop Keys :",
            );

            let right_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 2.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                setting.get_keys(TetrisCommand::Right),
                "Right Keys :",
            );

            let left_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 3.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                setting.get_keys(TetrisCommand::Left),
                "Left Keys :",
            );

            let rotate_clockwise_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH * 3.0 / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                setting.get_keys(TetrisCommand::Clockwise),
                "Rotate Clockwise Keys :",
            );

            let rotate_counterclockwise_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH * 3.0 / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                setting.get_keys(TetrisCommand::Counterclockwise),
                "Rotate Counterclockwise Keys :",
            );

            let rotate_half_turn_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH * 3.0 / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 2.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                setting.get_keys(TetrisCommand::HalfTurn),
                "Rotate Half Turn Keys :",
            );

            let hold_tetromino_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH * 3.0 / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 3.0,
                DEFAULT_KEY_INPUT_WIDTH,
                DEFAULT_KEY_INPUT_HEIGHT,
                setting.get_keys(TetrisCommand::Hold),
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

            horizontal_offset += DEFAULT_WINDOW_WIDTH;
        }

        let mut buttons = HashMap::new();
        if !from_game {
            let back_to_main_menu_button = Button::new_custom_size(
                (5.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
                (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
                DEFAULT_BUTTON_WIDTH / 6.0,
                DEFAULT_BUTTON_HEIGHT / 2.0,
                "Back",
            );
            buttons.insert(
                ViewChangeButton::BackToMainMenu.into(),
                back_to_main_menu_button,
            );
        } else {
            let back_to_game_button = Button::new_custom_size(
                (60.0 * DEFAULT_WINDOW_WIDTH + horizontal_offset) / 65.0,
                (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
                DEFAULT_BUTTON_WIDTH / 6.0,
                DEFAULT_BUTTON_HEIGHT / 2.0,
                "Back",
            );
            buttons.insert(ViewChangeButton::BackToGame.into(), back_to_game_button);
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
        let back_to_main_menu_button = Button::new_custom_size(
            (5.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let pause_button = Button::new_custom_size(
            (13.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 5.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Pause",
        );

        let settings_button = Button::new_custom_size(
            (58.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 3.5,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Settings",
        );

        let mut buttons = HashMap::new();
        buttons.insert(
            ViewChangeButton::BackToMainMenu.into(),
            back_to_main_menu_button,
        );
        buttons.insert(ViewChangeButton::ToPause.into(), pause_button);
        buttons.insert(ViewChangeButton::ToSettings.into(), settings_button);

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
        let back_to_main_menu_button = Button::new_custom_size(
            (5.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let pause_button = Button::new_custom_size(
            (13.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 5.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Pause",
        );

        let settings_button = Button::new_custom_size(
            (123.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 3.5,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Settings",
        );

        let mut buttons = HashMap::new();
        buttons.insert(
            ViewChangeButton::BackToMainMenu.into(),
            back_to_main_menu_button,
        );
        buttons.insert(ViewChangeButton::ToPause.into(), pause_button);
        buttons.insert(ViewChangeButton::ToSettings.into(), settings_button);

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
        let back_to_main_menu_button = Button::new_custom_size(
            (5.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let copy_ip = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0,
            "Copy room IP",
        );

        let mut buttons = HashMap::new();
        buttons.insert(
            ViewChangeButton::BackToMainMenu.into(),
            back_to_main_menu_button,
        );
        buttons.insert(CurrentViewButton::CopyToClipboard.into(), copy_ip);

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
        let back_to_main_menu_button = Button::new_custom_size(
            (5.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
            "Back",
        );

        let join_room = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0,
            "Join room",
        );

        let room_ip_input = TextInput::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 3.0,
            "Join room",
        );

        let paste_ip = Button::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
            "Paste",
        );

        let mut buttons = HashMap::new();
        buttons.insert(
            ViewChangeButton::BackToMainMenu.into(),
            back_to_main_menu_button,
        );
        buttons.insert(CurrentViewButton::ToTwoRemoteGame.into(), join_room);
        buttons.insert(CurrentViewButton::PasteFromClipboard.into(), paste_ip);
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
}
