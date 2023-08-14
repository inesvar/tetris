use std::collections::HashMap;

use crate::settings::KEY_REPEAT_DELAY;
use piston::Key;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Keyboard {
    // TODO : is it necessary to store the boolean ?
    pressed: HashMap<Key, bool>,

    /// This is initialized on the key press then is decremented until it reaches 0 and long press is triggered
    first_press_repeat_countdown: HashMap<Key, u64>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            pressed: HashMap::new(),
            first_press_repeat_countdown: HashMap::new(),
        }
    }

    pub fn set_pressed(&mut self, key: Key) {
        self.pressed.insert(key, true);
        self.first_press_repeat_countdown
            .insert(key, KEY_REPEAT_DELAY);
    }

    pub fn set_released(&mut self, key: Key) {
        self.pressed.insert(key, false);
        self.first_press_repeat_countdown.remove(&key);
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        match self.pressed.get(&key) {
            Some(pressed) => *pressed,
            None => false,
        }
    }

    pub fn is_delay_pressed(&self, key: Key) -> bool {
        match self.first_press_repeat_countdown.get(&key) {
            Some(counter) => *counter == 0,
            None => false,
        }
    }

    pub fn is_any_pressed(&self, keys: &[Key]) -> bool {
        for key in keys {
            if self.is_pressed(*key) {
                println!("pressed : true");
                println!("delay pressed : {}", self.is_any_delay_pressed(keys));
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
        for counter in self.first_press_repeat_countdown.values_mut() {
            if *counter > 0 {
                *counter -= 1;
            }
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
