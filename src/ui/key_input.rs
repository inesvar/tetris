use crate::{
    keyboard::{key_to_string, keys_to_string},
    settings::TEXT_COLOR,
    ui::text::Text,
};
use graphics::color;
use piston::{Key, MouseButton};

pub struct KeyInput {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) keys: Vec<Key>,
    pub(in crate::ui) text: Text,
    pub(in crate::ui) cursor: String,
    pub(in crate::ui) info_text: Text,
    placeholder: String,
    is_focused: bool,
    pub(in crate::ui) animation_counter: u64,
}

impl KeyInput {
    pub fn new_with_info(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        keys: &[Key],
        info_text: &str,
    ) -> Self {
        let placeholder = &keys_to_string(keys);
        let mut vec_keys = vec![];
        for key in keys {
            vec_keys.push(*key);
        }
        KeyInput {
            x,
            y,
            width,
            height,
            keys: vec_keys,
            cursor: String::from(""),
            info_text: Text::new(info_text, 16, x, y, TEXT_COLOR),
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
        if !self.is_focused {
            return;
        }
        match key {
            Key::Backspace => {
                self.text.content.pop();
                while self.text.content.chars().count() > 0 {
                    match self
                        .text
                        .content
                        .get((self.text.content.len() - 1)..=(self.text.content.len() - 1))
                    {
                        Some(" ") => {
                            break;
                        }
                        _ => {}
                    }
                    self.text.content.pop();
                }
            }
            Key::Return => {
                self.is_focused = false;
                if self.text.content == "" {
                    self.text.set_text(String::from(&self.placeholder));
                }
            }
            _ => {
                self.text.content.push_str(&key_to_string(key));
            }
        }
    }

    pub fn get_focused(&self) -> bool {
        self.is_focused
    }
}
