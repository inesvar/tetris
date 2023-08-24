use crate::settings::KEY_REPEAT_DELAY;
use piston::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Keyboard {
    last_pressed_key: Key,
    /// the delay is initialized on key press, then is decremented until it reaches 0 and long press is triggered
    delay_to_long_press: HashMap<Key, u64>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            last_pressed_key: Key::A,
            delay_to_long_press: HashMap::new(),
        }
    }

    pub fn set_pressed(&mut self, key: Key) {
        self.last_pressed_key = key;
        self.delay_to_long_press.insert(key, KEY_REPEAT_DELAY);
    }

    pub fn set_released(&mut self, key: Key) {
        self.delay_to_long_press.remove(&key);
    }

    pub fn is_any_last_pressed(&self, keys: &[Key]) -> bool {
        for key in keys {
            if self.is_last_pressed(*key) {
                return true;
            }
        }
        false
    }

    pub fn is_any_delay_pressed(&self, keys: &[Key]) -> bool {
        for key in keys {
            if self.is_delay_pressed(*key) {
                return true;
            }
        }
        false
    }

    /// decrements the press delay countdown for all keys
    pub fn update(&mut self) {
        for counter in self.delay_to_long_press.values_mut() {
            if *counter > 0 {
                *counter -= 1;
            }
        }
    }

    fn is_last_pressed(&self, key: Key) -> bool {
        self.last_pressed_key == key
    }

    fn is_delay_pressed(&self, key: Key) -> bool {
        match self.delay_to_long_press.get(&key) {
            Some(counter) => *counter == 0,
            None => false,
        }
    }
}

pub fn key_to_string(key: Key) -> String {
    if key == Key::Unknown {
        return String::from("");
    }
    format!("{:?}, ", key)
}

pub fn keys_to_string(keys: &[Key]) -> String {
    let mut s = String::new();
    for key in keys {
        s.push_str(&key_to_string(*key));
    }
    s
}
