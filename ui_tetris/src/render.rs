//! Implement [RenderTetrisUi] for [Piston2dOpenGlRenderer] (rendering [InteractiveWidgetManager] composed of [Text], [TextInput], [KeyInput], [Button]).
use super::button::Button;
use super::interactive_widget_manager::InteractiveWidgetManager;
use super::key_input::KeyInput;
use super::text::Text;
use super::text_input::TextInput;
use super::CURSOR_BLINK_PERIOD;
use graphics::{color, CharacterCache, Transformed};
use render_tetris::Piston2dOpenGlRenderer;

/// Rendering an [InteractiveWidgetManager].
pub trait RenderTetrisUi {
    fn render_widget_manager(&mut self, manager: &InteractiveWidgetManager);
    fn render_text_replace_content(&mut self, text_style: &Text, content: &str);
    fn render_text(&mut self, text: &Text) {
        self.render_text_replace_content(text, text.get_text());
    }
    fn render_editable_text(&mut self, text: &Text, cursor: bool) {
        self.render_text_replace_content(text, text.get_editable_text(cursor));
    }
    fn render_text_input(&mut self, input: &TextInput);
    fn render_key_input(&mut self, input: &KeyInput);
    fn render_button(&mut self, button: &Button);
}

impl RenderTetrisUi for Piston2dOpenGlRenderer<'_> {
    fn render_text_replace_content(&mut self, text: &Text, content: &str) {
        let old_transform = self.transform;

        let font = if text.use_tetris_font {
            &mut self.assets.tetris_font
        } else {
            &mut self.assets.main_font
        };

        let char = font.character(text.view.font_size, 'A').unwrap();
        let top = char.top();

        let text_width = font.width(text.view.font_size, content).unwrap();
        self.transform = self
            .transform
            .trans(text.center_x - text_width / 2.0, text.center_y + top / 2.0);

        text.view
            .draw(
                content,
                font,
                &self.draw_state,
                self.transform,
                &mut self.gl,
            )
            .unwrap();

        self.transform = old_transform;
    }

    fn render_text_input(&mut self, input: &TextInput) {
        let old_transform = self.transform;
        let (center_x, center_y) = input.rect.get_center();
        self.transform = self.transform.trans(center_x, center_y);

        let color = if input.focused {
            color::RED
        } else {
            color::SILVER
        };

        let outline_rect = graphics::Rectangle::new_border(color, 1.0);
        outline_rect.draw(
            input.rect.get_dimensions(),
            &self.draw_state,
            self.transform,
            &mut self.gl,
        );

        if !input.focused && input.text.is_empty() {
            self.render_text_replace_content(&input.text, &input.placeholder);
        } else {
            let cursor = input.focused
                && self.elapsed_secs() % CURSOR_BLINK_PERIOD < CURSOR_BLINK_PERIOD / 2.0;
            self.render_editable_text(&input.text, cursor);
        }

        self.render_text(&input.info_text);

        self.transform = old_transform;
    }

    fn render_key_input(&mut self, key_input: &KeyInput) {
        let old_transform = self.transform;
        let (center_x, center_y) = key_input.rect.get_center();
        self.transform = self.transform.trans(center_x, center_y);

        let color = if key_input.focused {
            color::RED
        } else {
            color::SILVER
        };

        let outline_rect = graphics::Rectangle::new_border(color, 1.0);
        outline_rect.draw(
            key_input.rect.get_dimensions(),
            &self.draw_state,
            self.transform,
            &mut self.gl,
        );

        if key_input.focused
            && self.elapsed_secs() % CURSOR_BLINK_PERIOD < CURSOR_BLINK_PERIOD / 2.0
        {
            self.render_editable_text(&key_input.custom_text, true);
        } else if key_input.focused || key_input.custom {
            self.render_text(&key_input.custom_text);
        } else {
            self.render_text(&key_input.placeholder);
        }

        self.render_text(&key_input.info_text);

        self.transform = old_transform;
    }

    fn render_button(&mut self, button: &Button) {
        let rectangle = graphics::Rectangle::new(button.color());

        let old_transform = self.transform;
        let (center_x, center_y) = button.rect.get_center();
        self.transform = self.transform.trans(center_x, center_y);

        rectangle.draw(
            button.rect.get_dimensions(),
            &self.draw_state,
            self.transform,
            &mut self.gl,
        );

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
