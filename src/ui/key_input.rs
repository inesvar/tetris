use crate::{settings::TEXT_COLOR, ui::text::Text};
use piston::{Key, MouseButton};

pub struct KeyInput {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) width: f64,
    pub(super) height: f64,
    pub(super) focused: bool, // true : display custom_text and cursor, false : depends on custom
    pub(super) custom: bool, // when unfocused, true : display custom_text, false : display keys_to_string(init_keys)
    pub(super) custom_text: Text,
    pub(super) cursor: String,
    pub(super) keys: Vec<Key>,
    init_keys: Vec<Key>,          // initial values from settings.rs
    pub(super) placeholder: Text, // initial text
    pub(super) commit: bool,      // true : update app's settings
    pub(super) info_text: Text,
    pub(super) animation_counter: u64,
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
        let placeholder: &str = &*keys_to_string(&keys);
        let vec_keys = keys.to_vec();
        KeyInput {
            x,
            y,
            width,
            height,
            focused: false,
            custom: false,
            custom_text: Text::new("", 16, x, y, TEXT_COLOR),
            cursor: String::from(""),
            keys: vec![],
            init_keys: vec_keys,
            placeholder: Text::new(placeholder, 16, x, y, TEXT_COLOR),
            commit: false,
            info_text: Text::new(info_text, 16, x, y, TEXT_COLOR),
            animation_counter: 0,
        }
    }

    pub fn are_coords_inside_input(&self, x: f64, y: f64) -> bool {
        x >= self.x - self.width / 2.0
            && x <= self.x + self.width / 2.0
            && y >= self.y - self.height / 2.0
            && y <= self.y + self.height / 2.0
    }

    pub fn commit(&mut self) -> bool {
        if self.commit {
            self.commit = false;
            true
        } else {
            false
        }
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton, cursor_position: &[f64; 2]) {
        match button {
            MouseButton::Left => {
                if self.are_coords_inside_input(cursor_position[0], cursor_position[1]) {
                    self.focus();
                } else if self.focused {
                    self.unfocus();
                }
            }
            _ => {}
        };
    }

    pub fn handle_key_press(&mut self, key: Key) {
        if !self.focused {
            return;
        }
        match key {
            Key::Backspace => {
                self.pop_key();
            }
            Key::Return => {
                self.unfocus();
            }
            _ => {
                self.push_key(key);
            }
        }
    }

    fn push_key(&mut self, key: Key) {
        self.custom_text.content.push_str(&key_to_string(key));
        self.keys.push(key);
    }

    fn pop_key(&mut self) {
        self.custom_text.content.pop();
        while self.custom_text.content.chars().count() > 0 {
            // TODO change this syntax
            match self
                .custom_text
                .content
                .get((self.custom_text.content.len() - 1)..=(self.custom_text.content.len() - 1))
            {
                Some(" ") => {
                    break;
                }
                _ => {}
            }
            self.custom_text.content.pop();
        }
        self.keys.pop();
    }

    fn unfocus(&mut self) {
        self.focused = false;
        if self.custom_text.content == "" {
            self.custom = false;
            self.custom_text.set_text(keys_to_string(&self.init_keys));
            self.keys = self.init_keys.clone();
        } else {
            self.custom = true;
        }
        self.commit = true;
    }

    fn focus(&mut self) {
        self.focused = true;
        if !self.custom {
            self.custom_text.set_text("".to_string());
            self.keys = vec![];
        }
    }
}

fn key_to_string(key: Key) -> String {
    if key == Key::Unknown {
        return String::from("");
    }
    format!("{:?}, ", key)
}

fn keys_to_string(keys: &[Key]) -> String {
    let mut s = String::new();
    for key in keys {
        s.push_str(&key_to_string(*key));
    }
    s
}
