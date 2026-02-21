//! Implement [Render] for [Text] and [InteractiveWidgetManager] (composed of [TextInput], [KeyInput] and [Button]).
use super::super::Piston2dOpenGlRenderer;
use super::button::Button;
use super::interactive_widget_manager::InteractiveWidgetManager;
use super::key_input::KeyInput;
use super::text::Text;
use super::text_input::TextInput;
use crate::{
    app::render_app::RenderTetrisUi,
    settings::{CURSOR_BLINK_PERIOD, DEFAULT_BUTTON_Y_SPACING, TEXT_COLOR},
};
use graphics::{color, rectangle, Transformed};

impl RenderTetrisUi for Piston2dOpenGlRenderer<'_> {
    fn render_text(&mut self, text: &Text) {
        let old_transform = self.transform;
        self.transform = self.transform.trans(
            text.x - text.content.len() as f64 * text.font_size as f64 * 0.315,
            text.y + text.font_size as f64 * 0.41,
        );

        let font = if text.use_tetris_font {
            &mut self.assets.tetris_font
        } else {
            &mut self.assets.main_font
        };
        text.view
            .draw(
                text.content.as_str(),
                font,
                &self.draw_state,
                self.transform,
                &mut self.gl,
            )
            .unwrap();

        self.transform = old_transform;
    }

    fn render_text_input(&mut self, input: &TextInput) {
        let dims = rectangle::rectangle_by_corners(
            -input.width / 2.0,
            -input.height / 2.0,
            input.width / 2.0,
            input.height / 2.0,
        );
        let button_transform = self.transform.trans(input.x, input.y);

        let color = if input.focused {
            color::RED
        } else {
            TEXT_COLOR
        };

        let outline_rect = graphics::Rectangle::new_border(color, 1.0);
        outline_rect.draw(dims, &self.draw_state, button_transform, &mut self.gl);

        if input.focused && self.elapsed_secs % CURSOR_BLINK_PERIOD < CURSOR_BLINK_PERIOD / 2.0 {
            let mut text_with_cursor = input.text.clone();
            text_with_cursor.content.push('|');
            self.render_text(&text_with_cursor);
        } else {
            self.render_text(&input.text);
        }

        let old_transform = self.transform;
        self.transform = self.transform.trans(0.0, -DEFAULT_BUTTON_Y_SPACING / 2.0);
        self.render_text(&input.info_text);

        self.transform = old_transform;
    }

    fn render_key_input(&mut self, key_input: &KeyInput) {
        let dims = rectangle::rectangle_by_corners(
            -key_input.width / 2.0,
            -key_input.height / 2.0,
            key_input.width / 2.0,
            key_input.height / 2.0,
        );
        let button_transform = self.transform.trans(key_input.x, key_input.y);

        let color = if key_input.focused {
            color::RED
        } else {
            TEXT_COLOR
        };

        let outline_rect = graphics::Rectangle::new_border(color, 1.0);
        outline_rect.draw(dims, &self.draw_state, button_transform, &mut self.gl);

        if key_input.focused && self.elapsed_secs % CURSOR_BLINK_PERIOD < CURSOR_BLINK_PERIOD / 2.0
        {
            let mut text_with_cursor = key_input.custom_text.clone();
            text_with_cursor.content.push('|');
            self.render_text(&text_with_cursor);
        } else if key_input.focused || key_input.custom {
            self.render_text(&key_input.custom_text);
        } else {
            self.render_text(&key_input.placeholder);
        }

        // render the info_text
        let old_transform = self.transform;
        self.transform = self.transform.trans(0.0, -DEFAULT_BUTTON_Y_SPACING / 2.0);
        self.render_text(&key_input.info_text);

        self.transform = old_transform;
    }

    fn render_button(&mut self, button: &Button) {
        let dims = rectangle::rectangle_by_corners(
            -button.width / 2.0,
            -button.height / 2.0,
            button.width / 2.0,
            button.height / 2.0,
        );
        let rectangle = graphics::Rectangle::new(button.background_color);

        let old_transform = self.transform;
        self.transform = self.transform.trans(button.x, button.y);

        rectangle.draw(dims, &self.draw_state, self.transform, &mut self.gl);

        self.render_text(&button.text);

        self.transform = old_transform;
    }

    fn render_widget_manager(&mut self, manager: &InteractiveWidgetManager) {
        for button in manager.buttons.values() {
            self.render_button(button);
        }

        for text_input in manager.text_inputs.values() {
            self.render_text_input(text_input);
        }
        for key_input in manager.key_inputs.values() {
            self.render_key_input(key_input);
        }
    }
}
