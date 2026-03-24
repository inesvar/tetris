use super::text::Text;
use super::{DEFAULT_FONT_SIZE, TEXT_COLOR};
use piston::{Key, MouseButton};

pub struct TextInput {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) width: f64,
    pub(super) height: f64,
    pub(super) text: Text,
    pub(super) info_text: Text,
    pub(super) placeholder: String,
    pub(super) focused: bool,
}

impl TextInput {
    pub fn new(x: f64, y: f64, width: f64, height: f64, placeholder: &str) -> Self {
        TextInput {
            x,
            y,
            width,
            height,
            info_text: Text::new("", DEFAULT_FONT_SIZE, x, y, TEXT_COLOR),
            text: Text::new("", DEFAULT_FONT_SIZE, x, y, TEXT_COLOR),
            placeholder: String::from(placeholder),
            focused: false,
        }
    }

    #[allow(unused)]
    pub fn new_with_info(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        placeholder: &str,
        info_text: &str,
    ) -> Self {
        TextInput {
            x,
            y,
            width,
            height,
            info_text: Text::new(info_text, DEFAULT_FONT_SIZE, x, y, TEXT_COLOR),
            text: Text::new("", DEFAULT_FONT_SIZE, x, y, TEXT_COLOR),
            placeholder: String::from(placeholder),
            focused: false,
        }
    }

    pub fn are_coords_inside_input(&self, x: f64, y: f64) -> bool {
        x >= self.x - self.width / 2.0
            && x <= self.x + self.width / 2.0
            && y >= self.y - self.height / 2.0
            && y <= self.y + self.height / 2.0
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton, [cursor_x, cursor_y]: &[f64; 2]) {
        if button == MouseButton::Left {
            self.focused = self.are_coords_inside_input(*cursor_x, *cursor_y);
        }
    }

    pub fn handle_key_press(&mut self, key: Key) {
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

    pub fn handle_text_input(&mut self, text: &str) {
        if self.focused {
            self.text.content.push_str(text);
        }
    }
}
