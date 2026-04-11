use crate::text_input::TextInput;
use graphics::math::Scalar;
use piston::Key;

pub struct KeyInput {
    pub(super) text_input: TextInput,
    pub(super) keys: Vec<Key>,
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
        let text_input = TextInput::new_with_text_info(
            center_x,
            center_y,
            width,
            height,
            &placeholder,
            info_text,
        );
        KeyInput {
            text_input,
            keys: vec![],
        }
    }

    pub(super) fn handle_left_click(&mut self, cursor_position: &[Scalar; 2]) {
        self.text_input.handle_left_click(cursor_position);
    }

    pub(super) fn handle_key_press(&mut self, key: Key) {
        if self.text_input.focused {
            match key {
                Key::Backspace => {
                    self.pop_key();
                }
                Key::Return => {
                    self.text_input.handle_key_press(key);
                }
                _ => {
                    self.push_key(key);
                }
            }
        }
    }

    fn push_key(&mut self, key: Key) {
        if key != Key::Unknown {
            self.text_input.text.push_str(&format!("{key:?}, "));
            self.keys.push(key);
        }
    }

    fn pop_key(&mut self) {
        self.keys.pop();
        self.text_input.text.set_text(keys_to_string(&self.keys));
    }
}

fn keys_to_string(keys: &[Key]) -> String {
    let mut s = String::new();
    for key in keys {
        s.push_str(&format!("{:?}, ", *key));
    }
    s
}
