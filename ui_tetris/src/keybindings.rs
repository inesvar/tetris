use core_tetris::TetrisCommand;
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

#[derive(Debug, Default)]
pub struct Keybindings(HashMap<TetrisCommand, Vec<Key>>);

impl Keybindings {
    pub fn new<F>(values: F) -> Self
    where
        HashMap<TetrisCommand, Vec<Key>>: From<F>,
    {
        Self(HashMap::from(values))
    }

    pub fn new_local() -> Self {
        let keys = [
            (TetrisCommand::Fall, Vec::from(FALL_KEYS_1P)),
            (TetrisCommand::HardDrop, Vec::from(HARD_DROP_KEYS_1P)),
            (TetrisCommand::Right, Vec::from(RIGHT_KEYS_1P)),
            (TetrisCommand::Left, Vec::from(LEFT_KEYS_1P)),
            (TetrisCommand::Clockwise, Vec::from(CLOCKWISE_KEYS_1P)),
            (
                TetrisCommand::Counterclockwise,
                Vec::from(COUNTERCLOCKWISE_KEYS_1P),
            ),
            (TetrisCommand::HalfTurn, Vec::from(HALF_TURN_1P)),
            (TetrisCommand::Hold, Vec::from(HOLD_KEYS_1P)),
        ];

        Self::new(keys)
    }

    pub fn new_two_local(id: usize) -> Self {
        let keys = if id == 0 {
            [
                (TetrisCommand::Fall, vec![FALL_KEYS_2P[0]]),
                (TetrisCommand::HardDrop, vec![HARD_DROP_KEYS_2P[0]]),
                (TetrisCommand::Right, vec![RIGHT_KEYS_2P[0]]),
                (TetrisCommand::Left, vec![LEFT_KEYS_2P[0]]),
                (TetrisCommand::Clockwise, vec![CLOCKWISE_KEYS_2P[0]]),
                (
                    TetrisCommand::Counterclockwise,
                    vec![COUNTERCLOCKWISE_KEYS_2P[0]],
                ),
                (TetrisCommand::HalfTurn, vec![HALF_TURN_2P[0]]),
                (TetrisCommand::Hold, vec![HOLD_KEYS_2P[0]]),
            ]
        } else {
            [
                (TetrisCommand::Fall, vec![FALL_KEYS_2P[1]]),
                (TetrisCommand::HardDrop, vec![HARD_DROP_KEYS_2P[1]]),
                (TetrisCommand::Right, vec![RIGHT_KEYS_2P[1]]),
                (TetrisCommand::Left, vec![LEFT_KEYS_2P[1]]),
                (TetrisCommand::Clockwise, vec![CLOCKWISE_KEYS_2P[1]]),
                (
                    TetrisCommand::Counterclockwise,
                    vec![COUNTERCLOCKWISE_KEYS_2P[1]],
                ),
                (TetrisCommand::HalfTurn, vec![HALF_TURN_2P[1]]),
                (TetrisCommand::Hold, vec![HOLD_KEYS_2P[1]]),
            ]
        };
        Self::new(keys)
    }

    pub fn set_keys(&mut self, key_type: TetrisCommand, new_keys: Vec<Key>) {
        self.0.insert(key_type, new_keys);
    }

    pub fn get_keys(&self, key_type: TetrisCommand) -> &[Key] {
        self.0.get(&key_type).map(Vec::as_slice).unwrap_or_default()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TetrisCommand, &Vec<Key>)> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use core_tetris::TetrisCommand;

    use crate::Keybindings;

    #[test]
    fn get_keys_doesnt_panic_for_absent_command() {
        let keybindings = Keybindings::new([]);
        keybindings.get_keys(TetrisCommand::Hold);
    }
}
