use super::text::Text;
use super::Rectangle;
use super::{DEFAULT_BUTTON_Y_SPACING, DEFAULT_FONT_SIZE};
use graphics::color::SILVER;
use graphics::math::Scalar;
use piston::Key;

pub struct KeyInput {
    pub(super) rect: Rectangle,
    pub(super) focused: bool, // true : display custom_text and cursor, false : depends on custom
    pub(super) custom: bool, // when unfocused, true : display custom_text, false : display keys_to_string(init_keys)
    pub(super) custom_text: Text,
    pub(super) keys: Vec<Key>,
    init_keys: Vec<Key>,          // initial values from settings.rs
    pub(super) placeholder: Text, // initial text
    pub(super) commit: bool,      // true : update app's settings
    pub(super) info_text: Text,
}

impl KeyInput {
    pub(super) fn new_with_info(
        center_x: Scalar,
        center_y: Scalar,
        width: Scalar,
        height: Scalar,
        keys: &[Key],
        info_text: &str,
    ) -> Self {
        let placeholder: &str = &keys_to_string(keys);
        let vec_keys = keys.to_vec();
        KeyInput {
            rect: Rectangle::new(center_x, center_y, width, height),
            focused: false,
            custom: false,
            custom_text: Text::new("", DEFAULT_FONT_SIZE, 0.0, 0.0, SILVER),
            keys: vec![],
            init_keys: vec_keys,
            placeholder: Text::new(placeholder, DEFAULT_FONT_SIZE, 0.0, 0.0, SILVER),
            commit: false,
            info_text: Text::new(
                info_text,
                DEFAULT_FONT_SIZE,
                0.0,
                -DEFAULT_BUTTON_Y_SPACING / 2.0,
                SILVER,
            ),
        }
    }

    pub(super) fn commit(&mut self) -> bool {
        if self.commit {
            self.commit = false;
            true
        } else {
            false
        }
    }

    pub(super) fn handle_left_click(&mut self, cursor_position: &[Scalar; 2]) {
        if self.rect.contains(cursor_position) {
            self.focus();
        } else if self.focused {
            self.unfocus();
        }
    }

    pub(super) fn handle_key_press(&mut self, key: Key) {
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
            if let Some(" ") = self
                .custom_text
                .content
                .get((self.custom_text.content.len() - 1)..=(self.custom_text.content.len() - 1))
            {
                break;
            }
            self.custom_text.content.pop();
        }
        self.keys.pop();
    }

    fn unfocus(&mut self) {
        self.focused = false;
        if self.custom_text.content.is_empty() {
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
