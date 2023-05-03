use std::{collections::HashMap};

use piston::Key;

pub struct Keyboard {
    pressed: HashMap<Key, bool>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            pressed: HashMap::new(),
        }
    }

    pub fn set_pressed(&mut self, key: Key) {
        self.pressed.insert(key, true);
    }

    pub fn set_released(&mut self, key: Key) {
        self.pressed.insert(key, false);
    }   

    pub fn is_pressed(&self, key: Key) -> bool {
        match self.pressed.get(&key) {
            Some(pressed) => *pressed,
            None => false,
        }
    }

    pub fn is_any_pressed(&self, keys: &[Key]) -> bool {
        for key in keys {
            if self.is_pressed(*key) {
                return true;
            }
        }
        false
    }
}