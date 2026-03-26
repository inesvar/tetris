use core_tetris::{TetrisCommand, TetrominoMove};
use piston::Key;
use std::collections::HashMap;

const FALL_KEYS_1P: [Key; 2] = [Key::Down, Key::NumPad2];
const HARD_DROP_KEYS_1P: [Key; 2] = [Key::Space, Key::NumPad8];
const RIGHT_KEYS_1P: [Key; 2] = [Key::Right, Key::NumPad6];
const LEFT_KEYS_1P: [Key; 2] = [Key::Left, Key::NumPad4];
const CLOCKWISE_KEYS_1P: [Key; 2] = [Key::Up, Key::NumPad5];
const COUNTERCLOCKWISE_KEYS_1P: [Key; 2] = [Key::NumPad0, Key::NumPad7];
const HALF_TURN_1P: [Key; 1] = [Key::A];
const HOLD_KEYS_1P: [Key; 2] = [Key::C, Key::NumPadEnter];

// First value is for the player on the left, second value for the player on the right
const FALL_KEYS_2P: [Key; 2] = [Key::X, Key::NumPad2];
const HARD_DROP_KEYS_2P: [Key; 2] = [Key::Z, Key::NumPad8];
const RIGHT_KEYS_2P: [Key; 2] = [Key::D, Key::NumPad6];
const LEFT_KEYS_2P: [Key; 2] = [Key::A, Key::NumPad4];
const CLOCKWISE_KEYS_2P: [Key; 2] = [Key::S, Key::NumPad5];
const COUNTERCLOCKWISE_KEYS_2P: [Key; 2] = [Key::Q, Key::NumPad7];
const HALF_TURN_2P: [Key; 2] = [Key::CapsLock, Key::NumPadPlus];
const HOLD_KEYS_2P: [Key; 2] = [Key::Space, Key::NumPadEnter];

#[derive(Debug)]
pub struct Keybindings {
    keys_for_command: HashMap<TetrisCommand, Vec<Key>>,
}

impl Keybindings {
    fn new<const N: usize>(array: [(TetrisCommand, Vec<Key>); N]) -> Self {
        Self {
            keys_for_command: HashMap::from(array),
        }
    }

    pub fn new_local() -> Self {
        let keys = [
            (TetrominoMove::Fall.into(), Vec::from(FALL_KEYS_1P)),
            (TetrominoMove::HardDrop.into(), Vec::from(HARD_DROP_KEYS_1P)),
            (TetrominoMove::Right.into(), Vec::from(RIGHT_KEYS_1P)),
            (TetrominoMove::Left.into(), Vec::from(LEFT_KEYS_1P)),
            (
                TetrominoMove::Clockwise.into(),
                Vec::from(CLOCKWISE_KEYS_1P),
            ),
            (
                TetrominoMove::Counterclockwise.into(),
                Vec::from(COUNTERCLOCKWISE_KEYS_1P),
            ),
            (TetrominoMove::HalfTurn.into(), Vec::from(HALF_TURN_1P)),
            (TetrisCommand::Hold, Vec::from(HOLD_KEYS_1P)),
        ];

        Self::new(keys)
    }

    pub fn new_two_local(id: usize) -> Self {
        let keys = if id == 0 {
            [
                (TetrominoMove::Fall.into(), vec![FALL_KEYS_2P[0]]),
                (TetrominoMove::HardDrop.into(), vec![HARD_DROP_KEYS_2P[0]]),
                (TetrominoMove::Right.into(), vec![RIGHT_KEYS_2P[0]]),
                (TetrominoMove::Left.into(), vec![LEFT_KEYS_2P[0]]),
                (TetrominoMove::Clockwise.into(), vec![CLOCKWISE_KEYS_2P[0]]),
                (
                    TetrominoMove::Counterclockwise.into(),
                    vec![COUNTERCLOCKWISE_KEYS_2P[0]],
                ),
                (TetrominoMove::HalfTurn.into(), vec![HALF_TURN_2P[0]]),
                (TetrisCommand::Hold, vec![HOLD_KEYS_2P[0]]),
            ]
        } else {
            [
                (TetrominoMove::Fall.into(), vec![FALL_KEYS_2P[1]]),
                (TetrominoMove::HardDrop.into(), vec![HARD_DROP_KEYS_2P[1]]),
                (TetrominoMove::Right.into(), vec![RIGHT_KEYS_2P[1]]),
                (TetrominoMove::Left.into(), vec![LEFT_KEYS_2P[1]]),
                (TetrominoMove::Clockwise.into(), vec![CLOCKWISE_KEYS_2P[1]]),
                (
                    TetrominoMove::Counterclockwise.into(),
                    vec![COUNTERCLOCKWISE_KEYS_2P[1]],
                ),
                (TetrominoMove::HalfTurn.into(), vec![HALF_TURN_2P[1]]),
                (TetrisCommand::Hold, vec![HOLD_KEYS_2P[1]]),
            ]
        };
        Self::new(keys)
    }

    pub fn set_keys(&mut self, key_type: &TetrisCommand, new_keys: Vec<Key>) {
        self.keys_for_command.insert(key_type.clone(), new_keys);
    }

    pub fn get_keys(&self, key_type: &TetrisCommand) -> &[Key] {
        self.keys_for_command.get(key_type).map(Vec::as_slice).unwrap_or_default()
    }

    pub fn get_commands(&self) -> impl Iterator<Item = &TetrisCommand> {
        self.keys_for_command.keys()
    }
}

#[cfg(test)]
mod tests {
    use core_tetris::TetrisCommand;

    use crate::Keybindings;

    #[test]
    fn get_keys_doesnt_panic_for_absent_command() {
        let keybindings = Keybindings::new([]);
        keybindings.get_keys(&TetrisCommand::Hold);
    }
}