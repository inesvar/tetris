use super::text::Text;
use super::Rectangle;
use super::{DEFAULT_BUTTON_Y_SPACING, DEFAULT_FONT_SIZE, TEXT_COLOR};
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
        center_x: f64,
        center_y: f64,
        width: f64,
        height: f64,
        placeholder: &str,
    ) -> Self {
        TextInput {
            rect: Rectangle::new(center_x, center_y, width, height),
            info_text: Text::new(
                "",
                DEFAULT_FONT_SIZE,
                0.0,
                -DEFAULT_BUTTON_Y_SPACING / 2.0,
                TEXT_COLOR,
            ),
            text: Text::new("", DEFAULT_FONT_SIZE, 0.0, 0.0, TEXT_COLOR),
            placeholder: String::from(placeholder),
            focused: false,
        }
    }

    pub(super) fn handle_left_click(&mut self, cursor: &[f64; 2]) {
        self.focused = self.rect.contains(cursor);
    }

    pub(super) fn handle_key_press(&mut self, key: Key) {
        if self.focused {
            match key {
                Key::Backspace => {
                    self.text.content.pop();
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
            self.text.content.push_str(text);
        }
    }
}
