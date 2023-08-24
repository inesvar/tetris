//! Defines [PressedKeys] that stores the pressed keys and the last pressed key.
use crate::settings::KEY_REPEAT_DELAY;
use piston::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pressed keys struct.
#[derive(Serialize, Deserialize)]
pub(super) struct PressedKeys {
    last_pressed_key: Key,
    /// the delay is initialized on key press, then is decremented until it reaches 0 and long press is triggered.
    delay_to_long_press: HashMap<Key, u64>,
}

impl PressedKeys {
    pub(super) fn new() -> PressedKeys {
        PressedKeys {
            last_pressed_key: Key::A,
            delay_to_long_press: HashMap::new(),
        }
    }

    pub(super) fn set_pressed(&mut self, key: Key) {
        self.last_pressed_key = key;
        self.delay_to_long_press.insert(key, KEY_REPEAT_DELAY);
    }

    pub(super) fn set_released(&mut self, key: Key) {
        self.delay_to_long_press.remove(&key);
    }

    pub(super) fn is_any_last_pressed(&self, keys: &[Key]) -> bool {
        for key in keys {
            if self.is_last_pressed(*key) {
                return true;
            }
        }
        false
    }

    pub(super) fn is_any_delay_pressed(&self, keys: &[Key]) -> bool {
        for key in keys {
            if self.is_delay_pressed(*key) {
                return true;
            }
        }
        false
    }

    /// decrements the press delay countdown for all keys
    pub(super) fn update(&mut self) {
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
