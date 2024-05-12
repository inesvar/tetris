use crate::{
    settings::{DEFAULT_FONT_SIZE, TEXT_COLOR},
    ui::text::Text,
};
use piston::{Key, MouseButton};

pub struct TextInput {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) width: f64,
    pub(super) height: f64,
    pub(super) text: Text,
    pub(super) cursor: String,
    pub(super) info_text: Text,
    placeholder: String,
    pub(super) focused: bool,
    pub(super) animation_counter: u64,
}

impl TextInput {
    pub fn new(x: f64, y: f64, width: f64, height: f64, placeholder: &str) -> Self {
        TextInput {
            x,
            y,
            width,
            height,
            info_text: Text::new("", DEFAULT_FONT_SIZE, x, y, TEXT_COLOR),
            cursor: String::from(""),
            text: Text::new(placeholder, DEFAULT_FONT_SIZE, x, y, TEXT_COLOR),
            placeholder: String::from(placeholder),
            focused: false,
            animation_counter: 0,
        }
    }

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
            cursor: String::from(""),
            text: Text::new(placeholder, DEFAULT_FONT_SIZE, x, y, TEXT_COLOR),
            placeholder: String::from(placeholder),
            focused: false,
            animation_counter: 0,
        }
    }

    pub fn are_coords_inside_input(&self, x: f64, y: f64) -> bool {
        x >= self.x - self.width / 2.0
            && x <= self.x + self.width / 2.0
            && y >= self.y - self.height / 2.0
            && y <= self.y + self.height / 2.0
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton, cursor_position: &[f64; 2]) {
        if button == MouseButton::Left {
            if self.are_coords_inside_input(cursor_position[0], cursor_position[1]) {
                self.focused = true;
                if self.text.content == self.placeholder {
                    self.text.set_text(String::from(""));
                }
            } else {
                self.focused = false;
                if self.text.content.is_empty() {
                    self.text.set_text(String::from(&self.placeholder));
                }
            }
        };
    }

    pub fn handle_key_press(&mut self, key: Key) {
        if self.focused {
            match key {
                Key::Backspace => {
                    if !self.text.content.is_empty() {
                        self.text.content.pop();
                    }
                }
                Key::Return => {
                    self.focused = false;
                    if self.text.content.is_empty() {
                        self.text.set_text(String::from(&self.placeholder));
                    }
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
