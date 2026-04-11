use super::text::Text;
use super::Rectangle;
use super::{DEFAULT_BUTTON_Y_SPACING, DEFAULT_FONT_SIZE};
use graphics::color::SILVER;
use graphics::math::Scalar;
use piston::Key;

pub struct TextInput {
    pub(super) rect: Rectangle,
    pub(super) text: Text,
    pub(super) info_text: Text,
    pub(super) placeholder: String,
    pub(super) focused: bool,
}

impl TextInput {
    pub(super) fn new(
        center_x: Scalar,
        center_y: Scalar,
        width: Scalar,
        height: Scalar,
        placeholder: &str,
    ) -> Self {
        TextInput::new_with_text_info(center_x, center_y, width, height, placeholder, "")
    }

    pub(super) fn new_with_text_info(
        center_x: Scalar,
        center_y: Scalar,
        width: Scalar,
        height: Scalar,
        placeholder: &str,
        info_text: &str,
    ) -> Self {
        TextInput {
            rect: Rectangle::new(center_x, center_y, width, height),
            info_text: Text::new(
                info_text,
                DEFAULT_FONT_SIZE,
                0.0,
                -DEFAULT_BUTTON_Y_SPACING / 2.0,
                SILVER,
            ),
            text: Text::new("", DEFAULT_FONT_SIZE, 0.0, 0.0, SILVER),
            placeholder: String::from(placeholder),
            focused: false,
        }
    }

    pub(super) fn handle_left_click(&mut self, cursor_position: &[Scalar; 2]) {
        self.focused = self.rect.contains(cursor_position);
    }

    pub(super) fn handle_key_press(&mut self, key: Key) {
        if self.focused {
            match key {
                Key::Backspace => {
                    self.text.pop();
                }
                Key::Return => {
                    self.focused = false;
                }
                _ => {}
            }
        }
    }

    pub(super) fn handle_text_input(&mut self, text: &str) {
        if self.focused {
            self.text.push_str(text);
        }
    }
}
