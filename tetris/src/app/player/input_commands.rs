//! Define `struct` [InputCommands].
use crate::settings::{AUTO_REPEAT_DELAY, AUTO_REPEAT_SPEED};
use core_tetris::TetrisCommand;
use piston::Key;
use std::collections::HashMap;
use ui_tetris::Keybindings;

#[derive(Debug)]
struct KeyLookup(HashMap<Key, TetrisCommand>);

/// Stores relevant keyboard inputs in order to output the right [TetrisCommand]s at each `update`.
pub(super) struct InputCommands {
    started_at: HashMap<TetrisCommand, u64>,
    // Some inputs have to be ignored. A typical example is when both right and left
    // keys are long pressed (cf Guideline 5.2 Auto-Repeat).
    temporarily_ignored: HashMap<TetrisCommand, u64>,
    now: u64,
    key_lookup: KeyLookup,
}

impl InputCommands {
    pub(super) fn new(keybindings: Keybindings) -> InputCommands {
        let key_lookup = KeyLookup::from(&keybindings);
        InputCommands {
            started_at: HashMap::new(),
            temporarily_ignored: HashMap::new(),
            now: 0,
            key_lookup,
        }
    }

    pub(super) fn should_apply_command(&self, command: TetrisCommand) -> bool {
        match self.key_is_pressed_since(command) {
            None => false,
            Some(0) => true,
            Some(duration) => {
                command.has_auto_repeat()
                    && duration
                        .checked_sub(AUTO_REPEAT_DELAY)
                        .is_some_and(|duration| duration % AUTO_REPEAT_SPEED == 0)
            }
        }
    }

    pub(super) fn set_pressed(&mut self, key: Key) {
        if let Some(command) = self.key_lookup.get_command(&key) {
            self.started_at.insert(command, self.now);

            self.ignore_opposite_command(command);
        }
    }

    pub(super) fn set_released(&mut self, key: Key) {
        if let Some(command) = self.key_lookup.get_command(&key) {
            self.started_at.remove(&command);
            self.temporarily_ignored.remove(&command);

            self.unignore_opposite_command(command);
        }
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

    pub(super) fn get_keybindings(&self) -> Keybindings {
        Keybindings::from(&self.key_lookup)
    }

    pub(super) fn set_new_keybindings(&mut self, keybindings: &Keybindings) {
        self.key_lookup = KeyLookup::from(keybindings);
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

impl From<&Keybindings> for KeyLookup {
    fn from(keybindings: &Keybindings) -> KeyLookup {
        let mut lookup = HashMap::new();
        for (command, keys) in keybindings.iter() {
            for key in keys {
                lookup.insert(*key, *command);
            }
        }

        KeyLookup(lookup)
    }
}

impl From<&KeyLookup> for Keybindings {
    fn from(key_lookup: &KeyLookup) -> Keybindings {
        let mut keybindings: HashMap<TetrisCommand, Vec<Key>> = HashMap::new();
        for (key, command) in key_lookup.iter() {
            keybindings.entry(*command).or_default().push(*key)
        }

        Keybindings::new(keybindings)
    }
}

impl KeyLookup {
    pub fn get_command(&self, key: &Key) -> Option<TetrisCommand> {
        self.0.get(key).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &TetrisCommand)> {
        self.0.iter()
    }
}

fn has_opposite_command(command: TetrisCommand) -> Option<TetrisCommand> {
    match command {
        TetrisCommand::Left => Some(TetrisCommand::Right),
        TetrisCommand::Right => Some(TetrisCommand::Left),
        _ => None,
    }
}

// TODO: write unit tests
