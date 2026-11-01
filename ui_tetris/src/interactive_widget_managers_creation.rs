use super::interactive_widget_manager::{
    ButtonType, CurrentViewButton, InteractiveWidgetManager, TextInputType, TextType,
    ViewChangeButton,
};
use super::{button::Button, key_input::KeyInput, text::Text, text_input::TextInput};
use super::{
    DEFAULT_BUTTON_HEIGHT, DEFAULT_BUTTON_WIDTH, DEFAULT_BUTTON_Y_SPACING, DEFAULT_FONT_SIZE,
    DEFAULT_SCORE_TEXT_Y, DEFAULT_TITLE_Y, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, SILVER,
};
use crate::keybindings::Keybindings;
use core_tetris::TetrisCommand;
use graphics::math::Scalar;
use render_tetris::{BLOCK_SIZE, DEFAULT_GRID_X};
use std::collections::HashMap;

impl InteractiveWidgetManager {
    pub fn new_main_menu() -> InteractiveWidgetManager {
        let buttons = HashMap::new();
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

        let mut manager = InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
            texts,
        };

        manager.add_central_column_buttons(
            [
                ViewChangeButton::ToSinglePlayerGame,
                ViewChangeButton::ToCreateRoom,
                ViewChangeButton::ToJoinRoom,
                ViewChangeButton::ToSettings,
                ViewChangeButton::ToTwoLocalGame,
            ],
            1,
        );

        manager
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
                setting.get_keys(TetrisCommand::SoftDrop),
                "Soft Drop Keys :",
            );

            let hard_drop_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
                setting.get_keys(TetrisCommand::HardDrop),
                "Hard Drop Keys :",
            );

            let right_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 2.0,
                setting.get_keys(TetrisCommand::Right),
                "Right Keys :",
            );

            let left_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 3.0,
                setting.get_keys(TetrisCommand::Left),
                "Left Keys :",
            );

            let rotate_clockwise_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH * 3.0 / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0,
                setting.get_keys(TetrisCommand::Clockwise),
                "Rotate Clockwise Keys :",
            );

            let rotate_counterclockwise_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH * 3.0 / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 1.0,
                setting.get_keys(TetrisCommand::Counterclockwise),
                "Rotate Counterclockwise Keys :",
            );

            let rotate_half_turn_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH * 3.0 / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 2.0,
                setting.get_keys(TetrisCommand::HalfTurn),
                "Rotate Half Turn Keys :",
            );

            let hold_tetromino_keys_input = KeyInput::new_with_info(
                DEFAULT_WINDOW_WIDTH * 3.0 / 4.0 + horizontal_offset,
                DEFAULT_WINDOW_HEIGHT / 2.0 + DEFAULT_BUTTON_Y_SPACING * 3.0,
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

        let buttons = HashMap::new();

        let title_text = Text::new_with_tetris_font(
            "T",
            DEFAULT_FONT_SIZE,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );

        let mut texts = HashMap::new();
        texts.insert(TextType::Title, title_text);

        let mut manager = InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
            texts,
        };

        if !from_game {
            manager.add_back_to_menu_button();
        } else {
            manager.add_button_custom_size(
                ViewChangeButton::BackToGame,
                (60.0 * DEFAULT_WINDOW_WIDTH + horizontal_offset) / 65.0,
                (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
                DEFAULT_BUTTON_WIDTH / 6.0,
                DEFAULT_BUTTON_HEIGHT / 2.0,
            );
        }

        manager
    }

    pub fn new_single_player_game() -> InteractiveWidgetManager {
        let buttons = HashMap::new();

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

        let mut manager = InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
            texts,
        };

        manager.add_back_to_menu_button();

        manager.add_button_custom_size(
            ViewChangeButton::ToPause,
            (13.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 5.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
        );

        manager.add_button_custom_size(
            ViewChangeButton::ToSettings,
            (58.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 3.5,
            DEFAULT_BUTTON_HEIGHT / 2.0,
        );

        manager
    }

    pub fn new_two_player_game() -> InteractiveWidgetManager {
        let buttons = HashMap::new();

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

        let mut manager = InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
            texts,
        };

        manager.add_back_to_menu_button();

        manager.add_button_custom_size(
            ViewChangeButton::ToPause,
            (13.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 5.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
        );

        manager.add_button_custom_size(
            ViewChangeButton::ToSettings,
            (123.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 3.5,
            DEFAULT_BUTTON_HEIGHT / 2.0,
        );

        manager
    }

    pub fn new_create_room() -> InteractiveWidgetManager {
        let buttons = HashMap::new();

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

        let mut manager = InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
            texts,
        };

        manager.add_back_to_menu_button();

        manager.add_central_column_buttons([CurrentViewButton::CopyToClipboard], 0);

        manager
    }

    pub fn new_join_room() -> InteractiveWidgetManager {
        let buttons = HashMap::new();

        let room_ip_input = TextInput::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 3.0,
            "Join room",
        );

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

        let mut manager = InteractiveWidgetManager {
            buttons,
            text_inputs,
            key_inputs,
            texts,
        };

        manager.add_back_to_menu_button();
        manager.add_central_column_buttons(
            [
                CurrentViewButton::ToTwoRemoteGame,
                CurrentViewButton::PasteFromClipboard,
            ],
            0,
        );

        manager
    }

    fn add_back_to_menu_button(&mut self) {
        self.add_button_custom_size(
            ViewChangeButton::BackToMainMenu,
            (5.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            (5.0 * DEFAULT_WINDOW_HEIGHT) / 70.0,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
        );
    }

    fn add_central_column_buttons(
        &mut self,
        buttons: impl IntoIterator<Item = impl Into<ButtonType>>,
        nb_buttons_above_middle: i32,
    ) {
        for (pos, button_type) in buttons.into_iter().enumerate() {
            self.add_button(
                button_type,
                DEFAULT_WINDOW_WIDTH / 2.0,
                DEFAULT_WINDOW_HEIGHT / 2.0
                    + DEFAULT_BUTTON_Y_SPACING * (pos as i32 - nb_buttons_above_middle) as f64,
            );
        }
    }

    fn add_button(
        &mut self,
        button_type: impl Into<ButtonType>,
        center_x: Scalar,
        center_y: Scalar,
    ) {
        let button_type = button_type.into();
        let button = Button::new(center_x, center_y, &button_type.to_string());
        self.buttons.insert(button_type, button);
    }

    fn add_button_custom_size(
        &mut self,
        button_type: impl Into<ButtonType>,
        center_x: Scalar,
        center_y: Scalar,
        width: Scalar,
        height: Scalar,
    ) {
        let button_type = button_type.into();
        let button =
            Button::new_custom_size(center_x, center_y, width, height, &button_type.to_string());
        self.buttons.insert(button_type, button);
    }
}
