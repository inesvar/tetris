//! Define [PressedKeys] that stores the pressed keys and the last pressed key.
use crate::settings::{AUTO_REPEAT_DELAY, AUTO_REPEAT_SPEED};
use core_tetris::{TetrisCommand, TetrominoMove};
use piston::Key;
use std::collections::HashMap;
use ui_tetris::{KeyLookup, Keybindings};

/// Maps keyboard inputs to [TetrisCommand]s and remembers durations so that auto-repeat can be applied.
pub(super) struct KeyboardInput {
    started_at: HashMap<TetrisCommand, u64>,
    /// Some inputs have to be ignored. A typical example is when both right and left
    /// keys are long pressed (cf Guideline 5.2 Auto-Repeat).
    temporarily_ignored: HashMap<TetrisCommand, u64>,
    now: u64,
    keybindings: Keybindings,
    key_lookup: KeyLookup,
}

impl KeyboardInput {
    pub(super) fn new(keybindings: Keybindings) -> KeyboardInput {
        let key_lookup = keybindings.build_key_lookup();
        KeyboardInput {
            started_at: HashMap::new(),
            temporarily_ignored: HashMap::new(),
            now: 0,
            keybindings,
            key_lookup,
        }
    }

    pub(super) fn set_pressed(&mut self, key: Key) -> Option<TetrisCommand> {
        let command = self.key_lookup.get_command(&key)?;
        self.started_at.insert(command, self.now);

        self.ignore_opposite_command(command);

        Some(command)
    }

    pub(super) fn set_released(&mut self, key: Key) {
        if let Some(command) = self.key_lookup.get_command(&key) {
            self.started_at.remove(&command);

            self.unignore_opposite_command(command);
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

    pub(super) fn get_keybindings(&self) -> &Keybindings {
        &self.keybindings
    }

    pub(super) fn get_mut_keybindings(&mut self) -> &mut Keybindings {
        &mut self.keybindings
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

    fn ignore_opposite_command(&mut self, command: TetrisCommand) {
        if let Some(opposite_command) = has_opposite_command(command) {
            let opposite_started = self.started_at.remove(&opposite_command);
            if let Some(opposite_started_at) = opposite_started {
                self.temporarily_ignored
                    .insert(opposite_command, opposite_started_at);
            }
        }
    }

    fn unignore_opposite_command(&mut self, command: TetrisCommand) {
        if let Some(opposite_command) = has_opposite_command(command) {
            let opposite_started = self.temporarily_ignored.remove(&opposite_command);
            if let Some(opposite_started_at) = opposite_started {
                self.started_at
                    .insert(opposite_command, opposite_started_at);
            }
        }
    }
}

fn has_opposite_command(command: TetrisCommand) -> Option<TetrisCommand> {
    match command {
        TetrisCommand::Move(TetrominoMove::Left) => Some(TetrominoMove::Right.into()),
        TetrisCommand::Move(TetrominoMove::Right) => Some(TetrominoMove::Left.into()),
        _ => None,
    }
}
