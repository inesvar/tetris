use super::text::Text;
use super::Rectangle;
use super::{DEFAULT_BUTTON_Y_SPACING, DEFAULT_FONT_SIZE};
use graphics::color::SILVER;
use graphics::math::Scalar;
use piston::Key;

pub struct KeyInput {
    pub(super) rect: Rectangle,
    pub(super) focused: bool, // true : display custom_text and cursor, false : depends on custom
    pub(super) custom_text: Text,
    pub(super) keys: Vec<Key>,      // initial values from settings.rs
    pub(super) placeholder: String, // initial text
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
        let placeholder = keys_to_string(keys);
        KeyInput {
            rect: Rectangle::new(center_x, center_y, width, height),
            focused: false,
            custom_text: Text::new("", DEFAULT_FONT_SIZE, 0.0, 0.0, SILVER),
            keys: vec![],
            placeholder,
            info_text: Text::new(
                info_text,
                DEFAULT_FONT_SIZE,
                0.0,
                -DEFAULT_BUTTON_Y_SPACING / 2.0,
                SILVER,
            ),
        }
    }

    pub(super) fn handle_left_click(&mut self, cursor_position: &[Scalar; 2]) {
        self.focused = self.rect.contains(cursor_position);
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
                self.focused = false;
            }
            _ => {
                self.push_key(key);
            }
        }
    }

    fn push_key(&mut self, key: Key) {
        self.custom_text.push_str(&key_to_string(key));
        self.keys.push(key);
    }

    fn pop_key(&mut self) {
        self.keys.pop();
        self.custom_text.set_text(keys_to_string(&self.keys));
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
