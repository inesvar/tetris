use super::interactive_widget_manager::{
    ButtonType, CurrentViewButton, InteractiveWidgetManager, TextInputType, TextType,
    ViewChangeButton,
};
use super::{button::Button, key_input::KeyInput, text::Text, text_input::TextInput};
use super::{
    BIGGER_FONT_SIZE, DEFAULT_BUTTON_HEIGHT, DEFAULT_BUTTON_WIDTH, DEFAULT_BUTTON_Y_SPACING,
    DEFAULT_FONT_SIZE, DEFAULT_SCORE_TEXT_Y, DEFAULT_TITLE_Y, DEFAULT_WINDOW_HEIGHT,
    DEFAULT_WINDOW_WIDTH, SILVER, TOP_LINE_HEIGHT,
};
use crate::keybindings::Keybindings;
use core_tetris::TetrisCommand;
use graphics::math::Scalar;
use render_tetris::{BLOCK_SIZE, DEFAULT_GRID_X};

impl InteractiveWidgetManager {
    pub fn new_main_menu() -> InteractiveWidgetManager {
        let mut manager = Self::new_tetris_view();

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
        let mut manager = Self::new_tetris_view();

        let mut horizontal_offset = 0.0;

        for (i, setting) in settings.enumerate() {
            manager.add_two_column_key_inputs(
                [
                    TetrisCommand::SoftDrop,
                    TetrisCommand::Clockwise,
                    TetrisCommand::HardDrop,
                    TetrisCommand::Counterclockwise,
                    TetrisCommand::Right,
                    TetrisCommand::HalfTurn,
                    TetrisCommand::Left,
                    TetrisCommand::Hold,
                ],
                i,
                horizontal_offset,
                &setting,
            );

            horizontal_offset += DEFAULT_WINDOW_WIDTH;
        }

        if !from_game {
            manager.add_back_to_menu_button();
        } else {
            manager.add_button_custom_size(
                ViewChangeButton::BackToGame,
                (60.0 * DEFAULT_WINDOW_WIDTH + horizontal_offset) / 65.0,
                TOP_LINE_HEIGHT,
                DEFAULT_BUTTON_WIDTH / 6.0,
                DEFAULT_BUTTON_HEIGHT / 2.0,
            );
        }

        manager
    }

    pub fn new_single_player_game() -> InteractiveWidgetManager {
        let mut manager = InteractiveWidgetManager::new_tetris_view();

        let restart_text = Text::new(
            "Press R to (re)start",
            BIGGER_FONT_SIZE,
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
            BIGGER_FONT_SIZE,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );

        manager.texts.insert(TextType::Restart, restart_text);
        manager.texts.insert(TextType::Timer, timer_text);
        manager.texts.insert(TextType::Pause, pause_text);

        manager.add_back_to_menu_button();

        manager.add_button_custom_size(
            ViewChangeButton::ToPause,
            (13.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            TOP_LINE_HEIGHT,
            DEFAULT_BUTTON_WIDTH / 5.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
        );

        manager.add_button_custom_size(
            ViewChangeButton::ToSettings,
            (58.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            TOP_LINE_HEIGHT,
            DEFAULT_BUTTON_WIDTH / 3.5,
            DEFAULT_BUTTON_HEIGHT / 2.0,
        );

        manager
    }

    pub fn new_two_player_game() -> InteractiveWidgetManager {
        let mut manager = InteractiveWidgetManager::new_tetris_view();

        let restart_text = Text::new(
            "Press R to (re)start",
            BIGGER_FONT_SIZE,
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
            BIGGER_FONT_SIZE,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );

        manager.texts.insert(TextType::Restart, restart_text);
        manager.texts.insert(TextType::Timer, timer_text);
        manager.texts.insert(TextType::Pause, pause_text);

        manager.add_back_to_menu_button();

        manager.add_button_custom_size(
            ViewChangeButton::ToPause,
            (13.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            TOP_LINE_HEIGHT,
            DEFAULT_BUTTON_WIDTH / 5.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
        );

        manager.add_button_custom_size(
            ViewChangeButton::ToSettings,
            (123.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            TOP_LINE_HEIGHT,
            DEFAULT_BUTTON_WIDTH / 3.5,
            DEFAULT_BUTTON_HEIGHT / 2.0,
        );

        manager
    }

    pub fn new_create_room() -> InteractiveWidgetManager {
        let mut manager = Self::new_tetris_view();

        manager.add_back_to_menu_button();
        manager.add_central_column_buttons([CurrentViewButton::CopyToClipboard], 0);

        manager
    }

    pub fn new_join_room() -> InteractiveWidgetManager {
        let mut manager = InteractiveWidgetManager::new_tetris_view();

        let room_ip_input = TextInput::new(
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_WINDOW_HEIGHT / 3.0,
            "Join room",
        );

        manager
            .text_inputs
            .insert(TextInputType::IpAddressInput, room_ip_input);

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

    fn new_tetris_view() -> Self {
        let mut manager = Self::default();
        manager.add_title_text();
        manager
    }

    fn add_back_to_menu_button(&mut self) {
        self.add_button_custom_size(
            ViewChangeButton::BackToMainMenu,
            (5.0 * DEFAULT_WINDOW_WIDTH) / 65.0,
            TOP_LINE_HEIGHT,
            DEFAULT_BUTTON_WIDTH / 6.0,
            DEFAULT_BUTTON_HEIGHT / 2.0,
        );
    }

    fn add_title_text(&mut self) {
        let title_text = Text::new_with_tetris_font(
            "T",
            DEFAULT_FONT_SIZE,
            DEFAULT_WINDOW_WIDTH / 2.0,
            DEFAULT_TITLE_Y,
            SILVER,
        );

        self.texts.insert(TextType::Title, title_text);
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
                    + DEFAULT_BUTTON_Y_SPACING * (pos as f64 - nb_buttons_above_middle as f64),
            );
        }
    }

    fn add_two_column_key_inputs(
        &mut self,
        key_inputs: impl IntoIterator<Item = TetrisCommand>,
        player_id: usize,
        horizontal_offset: Scalar,
        keybindings: &Keybindings,
    ) {
        for (index, command) in key_inputs.into_iter().enumerate() {
            let (x, y) = grid_cell_center(index, 2);
            self.add_key_input(command, player_id, x + horizontal_offset, y, keybindings);
        }
    }

    fn add_key_input(
        &mut self,
        command: TetrisCommand,
        player: usize,
        center_x: Scalar,
        center_y: Scalar,
        keybindings: &Keybindings,
    ) {
        let key_input = KeyInput::new_with_info(
            center_x,
            center_y,
            keybindings.get_keys(command),
            &format!("{command} Keys :"),
        );
        self.key_inputs.insert((command, player), key_input);
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

fn grid_cell_center(index: usize, nb_columns: usize) -> (Scalar, Scalar) {
    let nb_columns = nb_columns.max(1);
    let column = index % nb_columns;
    let row = index / nb_columns;
    (
        DEFAULT_WINDOW_WIDTH / (2.0 * nb_columns as Scalar)
            + column as Scalar * (DEFAULT_WINDOW_WIDTH / nb_columns as Scalar),
        DEFAULT_WINDOW_HEIGHT / 2.0 + row as Scalar * DEFAULT_BUTTON_Y_SPACING,
    )
}
