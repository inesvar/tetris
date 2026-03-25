use core_tetris::{TetrisCommand, TetrominoMove};
use piston::Key;

const FALL_KEYS_1P: [Key; 2] = [Key::Down, Key::NumPad2];
const HARD_DROP_KEYS_1P: [Key; 2] = [Key::Space, Key::NumPad8];
const RIGHT_KEYS_1P: [Key; 2] = [Key::Right, Key::NumPad6];
const LEFT_KEYS_1P: [Key; 2] = [Key::Left, Key::NumPad4];
const ROTATE_CLOCKWISE_KEYS_1P: [Key; 2] = [Key::Up, Key::NumPad5];
const ROTATE_COUNTERCLOCKWISE_KEYS_1P: [Key; 2] = [Key::NumPad0, Key::NumPad7];
const ROTATE_HALF_TURN_1P: [Key; 1] = [Key::A];
const HOLD_TETROMINO_KEYS_1P: [Key; 2] = [Key::C, Key::NumPadEnter];

// First value is for the player on the left, second value for the player on the right
const FALL_KEYS_2P: [Key; 2] = [Key::X, Key::NumPad2];
const HARD_DROP_KEYS_2P: [Key; 2] = [Key::Z, Key::NumPad8];
const RIGHT_KEYS_2P: [Key; 2] = [Key::D, Key::NumPad6];
const LEFT_KEYS_2P: [Key; 2] = [Key::A, Key::NumPad4];
const ROTATE_CLOCKWISE_KEYS_2P: [Key; 2] = [Key::S, Key::NumPad5];
const ROTATE_COUNTERCLOCKWISE_KEYS_2P: [Key; 2] = [Key::Q, Key::NumPad7];
const ROTATE_HALF_TURN_2P: [Key; 2] = [Key::CapsLock, Key::NumPadPlus];
const HOLD_TETROMINO_KEYS_2P: [Key; 2] = [Key::Space, Key::NumPadEnter];

pub struct Keybindings {
    pub fall_keys: Vec<Key>,
    pub hard_drop_keys: Vec<Key>,
    pub right_keys: Vec<Key>,
    pub left_keys: Vec<Key>,
    pub rotate_clockwise_keys: Vec<Key>,
    pub rotate_counterclockwise_keys: Vec<Key>,
    pub rotate_half_turn_keys: Vec<Key>,
    pub hold_tetromino_keys: Vec<Key>,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self::new()
    }
}

impl Keybindings {
    pub fn new() -> Keybindings {
        let fall_keys = FALL_KEYS_1P.to_vec();
        let hard_drop_keys = HARD_DROP_KEYS_1P.to_vec();
        let right_keys = RIGHT_KEYS_1P.to_vec();
        let left_keys = LEFT_KEYS_1P.to_vec();
        let rotate_clockwise_keys = ROTATE_CLOCKWISE_KEYS_1P.to_vec();
        let rotate_counterclockwise_keys = ROTATE_COUNTERCLOCKWISE_KEYS_1P.to_vec();
        let rotate_half_turn_keys = ROTATE_HALF_TURN_1P.to_vec();
        let hold_tetromino_keys = HOLD_TETROMINO_KEYS_1P.to_vec();

        Keybindings {
            fall_keys,
            hard_drop_keys,
            right_keys,
            left_keys,
            rotate_clockwise_keys,
            rotate_counterclockwise_keys,
            rotate_half_turn_keys,
            hold_tetromino_keys,
        }
    }

    pub fn new_two_local(id: usize) -> Keybindings {
        if id == 0 {
            let fall_keys = vec![FALL_KEYS_2P[0]];
            let hard_drop_keys = vec![HARD_DROP_KEYS_2P[0]];
            let right_keys = vec![RIGHT_KEYS_2P[0]];
            let left_keys = vec![LEFT_KEYS_2P[0]];
            let rotate_clockwise_keys = vec![ROTATE_CLOCKWISE_KEYS_2P[0]];
            let rotate_counterclockwise_keys = vec![ROTATE_COUNTERCLOCKWISE_KEYS_2P[0]];
            let rotate_half_turn_keys = vec![ROTATE_HALF_TURN_2P[0]];
            let hold_tetromino_keys = vec![HOLD_TETROMINO_KEYS_2P[0]];

            Keybindings {
                fall_keys,
                hard_drop_keys,
                right_keys,
                left_keys,
                rotate_clockwise_keys,
                rotate_counterclockwise_keys,
                rotate_half_turn_keys,
                hold_tetromino_keys,
            }
        } else {
            let fall_keys = vec![FALL_KEYS_2P[1]];
            let hard_drop_keys = vec![HARD_DROP_KEYS_2P[1]];
            let right_keys = vec![RIGHT_KEYS_2P[1]];
            let left_keys = vec![LEFT_KEYS_2P[1]];
            let rotate_clockwise_keys = vec![ROTATE_CLOCKWISE_KEYS_2P[1]];
            let rotate_counterclockwise_keys = vec![ROTATE_COUNTERCLOCKWISE_KEYS_2P[1]];
            let rotate_half_turn_keys = vec![ROTATE_HALF_TURN_2P[1]];
            let hold_tetromino_keys = vec![HOLD_TETROMINO_KEYS_2P[1]];

            Keybindings {
                fall_keys,
                hard_drop_keys,
                right_keys,
                left_keys,
                rotate_clockwise_keys,
                rotate_counterclockwise_keys,
                rotate_half_turn_keys,
                hold_tetromino_keys,
            }
        }
    }

    pub fn set_keys(&mut self, key_type: &TetrisCommand, new_keys: Vec<Key>) {
        match key_type {
            TetrisCommand::Move(TetrominoMove::Fall) => self.fall_keys = new_keys,
            TetrisCommand::Move(TetrominoMove::HardDrop) => self.hard_drop_keys = new_keys,
            TetrisCommand::Move(TetrominoMove::Right) => self.right_keys = new_keys,
            TetrisCommand::Move(TetrominoMove::Left) => self.left_keys = new_keys,
            TetrisCommand::Move(TetrominoMove::Clockwise) => self.rotate_clockwise_keys = new_keys,
            TetrisCommand::Move(TetrominoMove::Counterclockwise) => {
                self.rotate_counterclockwise_keys = new_keys
            }
            TetrisCommand::Move(TetrominoMove::HalfTurn) => self.rotate_half_turn_keys = new_keys,
            TetrisCommand::Hold => self.hold_tetromino_keys = new_keys,
            _ => unreachable!(),
        }
    }

    pub fn print(&self) {
        println!("printing the updated settings");
        println!("     fall_keys: {:?}", self.fall_keys);
        println!("     hard_drop_keys: {:?}", self.hard_drop_keys);
        println!("     right_keys: {:?}", self.right_keys);
        println!("     left_keys: {:?}", self.left_keys);
        println!(
            "     rotate_clockwise_keys: {:?}",
            self.rotate_clockwise_keys
        );
        println!(
            "     rotate_counterclockwise_keys: {:?}",
            self.rotate_counterclockwise_keys
        );
        println!("     hold_tetromino_keys: {:?}", self.hold_tetromino_keys);
        println!();
    }
}
