use crate::{settings::TEXT_COLOR, ui::text::Text};
use graphics::color;
use piston::{Key, MouseButton};

pub struct TextInput {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(in crate::ui) text: Text,
    placeholder: String,
    is_focused: bool,
    pub(in crate::ui) animation_counter: u64,
}

impl TextInput {
    pub fn new(x: f64, y: f64, width: f64, height: f64, placeholder: &str) -> Self {
        TextInput {
            x,
            y,
            width,
            height,
            text: Text::new(placeholder, 16, x, y, TEXT_COLOR),
            placeholder: String::from(placeholder),
            is_focused: false,
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
        match button {
            MouseButton::Left => {
                if self.are_coords_inside_input(cursor_position[0], cursor_position[1]) {
                    self.is_focused = true;
                    if self.text.content == self.placeholder {
                        self.text.set_text(String::from(""));
                    }
                } else {
                    self.is_focused = false;
                    if self.text.content == "" {
                        self.text.set_text(String::from(&self.placeholder));
                    }
                }
            }
            _ => {}
        };
    }

    pub fn handle_key_press(&mut self, key: Key) {
        if self.is_focused {
            match key {
                Key::Backspace => {
                    if self.text.content.len() > 0 {
                        self.text.content.pop();
                    }
                }
                Key::Return => {
                    self.is_focused = false;
                    if self.text.content == "" {
                        self.text.set_text(String::from(&self.placeholder));
                    }
                }
                _ => {}
            }
        }
    }

    pub fn handle_text_input(&mut self, text: &String) {
        if self.is_focused {
            self.text.content.push_str(text);
        }
    }

    pub fn get_focused(&self) -> bool {
        self.is_focused
    }
}
