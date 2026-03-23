//! Define [PressedKeys] that stores the pressed keys and the last pressed key.
use crate::keybindings::Keybindings;
use crate::settings::{AUTO_REPEAT_DELAY, AUTO_REPEAT_SPEED};
use core_tetris::{TetrisCommand, TetrominoMove};
use piston::Key;
use std::collections::HashMap;

/// Pressed keys struct.
pub(super) struct KeyboardInput {
    started_at: HashMap<TetrisCommand, u64>,
    now: u64,
    keybindings: Keybindings,
}

impl KeyboardInput {
    pub(super) fn new(keybindings: Keybindings) -> KeyboardInput {
        KeyboardInput {
            started_at: HashMap::new(),
            now: 0,
            keybindings,
        }
    }

    pub(super) fn get_order_from_key(&self, key: Key) -> Option<TetrisCommand> {
        match key {
            key if self.keybindings.hold_tetromino_keys.contains(&key) => Some(TetrisCommand::Hold),
            key if self.keybindings.fall_keys.contains(&key) => Some(TetrominoMove::Fall.into()),
            key if self.keybindings.hard_drop_keys.contains(&key) => {
                Some(TetrominoMove::HardDrop.into())
            }
            key if self.keybindings.left_keys.contains(&key) => Some(TetrominoMove::Left.into()),
            key if self.keybindings.right_keys.contains(&key) => Some(TetrominoMove::Right.into()),
            key if self.keybindings.rotate_clockwise_keys.contains(&key) => {
                Some(TetrominoMove::Clockwise.into())
            }
            key if self.keybindings.rotate_counterclockwise_keys.contains(&key) => {
                Some(TetrominoMove::Counterclockwise.into())
            }
            key if self.keybindings.rotate_half_turn_keys.contains(&key) => {
                Some(TetrominoMove::HalfTurn.into())
            }
            _ => None,
        }
    }

    pub(super) fn get_keybindings(&self) -> &Keybindings {
        &self.keybindings
    }

    pub(super) fn get_mut_keybindings(&mut self) -> &mut Keybindings {
        &mut self.keybindings
    }

    pub(super) fn set_pressed(&mut self, key: Key) -> Option<TetrisCommand> {
        let command = self.get_order_from_key(key)?;
        self.started_at.insert(command.clone(), self.now);
        Some(command)
    }

    pub(super) fn set_released(&mut self, key: Key) {
        if let Some(command) = self.get_order_from_key(key) {
            self.started_at.remove(&command);
        }
    }

    pub(super) fn is_auto_repeated(&self, command: TetrisCommand) -> bool {
        self.auto_repeat_delay_is_up_since(command)
            .is_some_and(|x| x % AUTO_REPEAT_SPEED == 0)
    }

    /// Increments the timer count.
    pub(super) fn update(&mut self) {
        self.now = self.now.wrapping_add(1);
    }

    fn key_is_pressed_since(&self, command: TetrisCommand) -> Option<u64> {
        self.started_at
            .get(&command)
            .map(|pressed_at| self.now.wrapping_sub(*pressed_at))
    }

    fn auto_repeat_delay_is_up_since(&self, command: TetrisCommand) -> Option<u64> {
        match self.key_is_pressed_since(command) {
            None => None,
            Some(duration) => duration.checked_sub(AUTO_REPEAT_DELAY),
        }
    }
}
