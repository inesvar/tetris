//! Define [PressedKeys] that stores the pressed keys and the last pressed key.
use crate::settings::AUTO_REPEAT_DELAY;
use crate::{keybindings::Keybindings, settings::AUTO_REPEAT_SPEED};
use core_tetris::{TetrisCommand, TetrominoMove};
use piston::Key;
use std::collections::HashMap;

/// Pressed keys struct.
pub(super) struct PressedKeys {
    /// the countdown is initialized on a key press, then is decremented until it reaches 0 and long press is triggered.
    timer_countdown: HashMap<Key, u64>,
    update: u64,
}

pub fn get_order_from_key(keybindings: &Keybindings, key: Key) -> Option<TetrisCommand> {
    match key {
        key if keybindings.hold_tetromino_keys.contains(&key) => Some(TetrisCommand::Hold),
        key if keybindings.fall_keys.contains(&key) => Some(TetrominoMove::Fall.into()),
        key if keybindings.hard_drop_keys.contains(&key) => Some(TetrominoMove::HardDrop.into()),
        key if keybindings.left_keys.contains(&key) => Some(TetrominoMove::Left.into()),
        key if keybindings.right_keys.contains(&key) => Some(TetrominoMove::Right.into()),
        key if keybindings.rotate_clockwise_keys.contains(&key) => {
            Some(TetrominoMove::Clockwise.into())
        }
        key if keybindings.rotate_counterclockwise_keys.contains(&key) => {
            Some(TetrominoMove::Counterclockwise.into())
        }
        key if keybindings.rotate_half_turn_keys.contains(&key) => {
            Some(TetrominoMove::HalfTurn.into())
        }
        _ => None,
    }
}

impl PressedKeys {
    pub(super) fn new() -> PressedKeys {
        PressedKeys {
            timer_countdown: HashMap::new(),
            update: 0,
        }
    }

    pub(super) fn set_pressed(&mut self, key: Key) {
        self.timer_countdown.insert(key, self.update);
    }

    pub(super) fn set_released(&mut self, key: Key) {
        self.timer_countdown.remove(&key);
    }

    pub(super) fn is_auto_repeated(&self, keys: &[Key]) -> bool {
        self.auto_repeat_delay_is_up_since(keys)
            .is_some_and(|x| x % AUTO_REPEAT_SPEED == 0)
    }

    /// Increments the timer count.
    pub(super) fn update(&mut self) {
        self.update = self.update.wrapping_add(1);
    }

    fn key_is_pressed_since(&self, key: &Key) -> Option<u64> {
        self.timer_countdown
            .get(key)
            .map(|pressed_at| self.update.wrapping_sub(*pressed_at))
    }

    fn is_pressed_since(&self, keys: &[Key]) -> Option<u64> {
        keys.iter().fold(None, |acc, key| {
            let key_pressed_since = self.key_is_pressed_since(key);
            let Some(maximum) = acc else {
                return key_pressed_since;
            };
            if key_pressed_since.is_some_and(|pressed_since| pressed_since > maximum) {
                key_pressed_since
            } else {
                acc
            }
        })
    }

    fn auto_repeat_delay_is_up_since(&self, keys: &[Key]) -> Option<u64> {
        match self.is_pressed_since(keys) {
            None => None,
            Some(duration) => duration.checked_sub(AUTO_REPEAT_DELAY),
        }
    }
}
