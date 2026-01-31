//! Define [PressedKeys] that stores the pressed keys and the last pressed key.
use crate::keybindings::Keybindings;
use crate::settings::KEY_REPEAT_DELAY;
use piston::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tetris_core::{TetrisOrder, TetrominoMove};

// TODO :
// it would be more natural to have a hashmap associating a `Key` to an `Option<Command>`.
// The command could be moving the active tetromino, or a game command.

// And the long press should be handled by a lib obviously

/// Pressed keys struct.
#[derive(Serialize, Deserialize)]
pub(super) struct PressedKeys {
    last_pressed_key: Key,
    /// the countdown is initialized on a key press, then is decremented until it reaches 0 and long press is triggered.
    timer_countdown: HashMap<Key, u64>,
}

pub fn get_order_from_key(keybindings: &Keybindings, key: Key) -> Option<TetrisOrder> {
    match key {
        key if keybindings.hold_tetromino_keys.contains(&key) => {
            Some(TetrisOrder::PlayerStashesTetromino)
        }
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
            last_pressed_key: Key::A,
            timer_countdown: HashMap::new(),
        }
    }

    pub(super) fn set_pressed(&mut self, key: Key) {
        self.last_pressed_key = key;
        self.timer_countdown.insert(key, KEY_REPEAT_DELAY);
    }

    pub(super) fn set_released(&mut self, key: Key) {
        self.timer_countdown.remove(&key);
    }

    pub(super) fn was_just_pressed(&self, keys: &[Key]) -> bool {
        keys.contains(&self.last_pressed_key)
    }

    pub(super) fn is_long_pressed(&self, keys: &[Key]) -> bool {
        keys.iter().any(|k| self.is_delay_pressed(*k))
    }

    /// Decrements the press delay countdown for all pressed keys.
    pub(super) fn update(&mut self) {
        for countdown in self.timer_countdown.values_mut() {
            *countdown = countdown.saturating_sub(1);
        }
    }

    pub(super) fn last_pressed_key(&self) -> Key {
        self.last_pressed_key
    }

    fn is_delay_pressed(&self, key: Key) -> bool {
        self.timer_countdown
            .get(&key)
            .is_some_and(|countdown| *countdown == 0)
    }
}
