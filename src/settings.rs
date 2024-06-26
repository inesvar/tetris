use std::{cell::RefCell, net::TcpStream};

use crate::{once, ui::interactive_widget_manager::TetrisCommand, PlayerConfig};
use opengl_graphics::OpenGL;
use piston::Key;
use serde::Deserialize;

static SCALE_FACTOR: f64 = 1.0;
pub static HOST_PORT: &str = ":26000";
pub static GUEST_PORT: &str = ":26005";

// Change this to OpenGL::V2_1 if not working.
pub static OPENGL_VERSION: OpenGL = OpenGL::V4_5;

pub static NB_COLUMNS: u32 = 10;
pub static NB_ROWS: u32 = 22;

/****************************************/
/* POSITIONS IN SINGLE PLAYER GAME VIEW */
/****************************************/

pub static DEFAULT_WINDOW_WIDTH: u32 = (650.0 * SCALE_FACTOR) as u32;
pub static DEFAULT_WINDOW_HEIGHT: u32 = (800.0 * SCALE_FACTOR) as u32;

// coordinates of the top left corner of the grid
// changing this moves everything in the single-player view except the title
pub static DEFAULT_GRID_X: f64 = 200.0 * SCALE_FACTOR;
pub static DEFAULT_GRID_Y: f64 = 160.0 * SCALE_FACTOR;

// height of the title "Tetris" and "Press R to (re)start"
pub static DEFAULT_TITLE_Y: f64 = 100.0 * SCALE_FACTOR;
// height of the text on the left side indicating the score
pub static DEFAULT_SCORE_TEXT_Y: f64 = DEFAULT_GRID_Y + 8.0 * BLOCK_SIZE;

pub static BLOCK_SIZE: f64 = 25.0 * SCALE_FACTOR;
pub static TETROMINO_MAX_WIDTH: f64 = 4.0 * BLOCK_SIZE;
pub static TETROMINO_MAX_HEIGHT: f64 = 2.0 * BLOCK_SIZE;
pub static GRID_THICKNESS: f64 = 0.5;

/****************************************/
/*      POSITIONS IN MAIN MENU VIEW     */
/****************************************/

// size of the buttons
pub static DEFAULT_BUTTON_WIDTH: f64 = 300.0 * SCALE_FACTOR;
pub static DEFAULT_BUTTON_HEIGHT: f64 = 50.0 * SCALE_FACTOR;
pub static DEFAULT_BUTTON_Y_SPACING: f64 = 100.0 * SCALE_FACTOR;
// size of the text
pub static DEFAULT_FONT_SIZE: u32 = (16.0 * SCALE_FACTOR) as u32;

/****************************************/
/*      POSITIONS IN SETTINGS VIEW      */
/****************************************/

// size of the buttons
pub static DEFAULT_KEY_INPUT_WIDTH: f64 = 200.0 * SCALE_FACTOR;
pub static DEFAULT_KEY_INPUT_HEIGHT: f64 = 50.0 * SCALE_FACTOR;

/****************************************/
/*               COLORS                 */
/****************************************/

pub static BG_COLOR: graphics::types::Color = [0.0, 0.3, 0.1, 1.0];
pub static GRID_BG_COLOR: graphics::types::Color = [0.3, 0.3, 0.3, 1.0];
pub static GRID_COLOR: graphics::types::Color = [0.8, 0.8, 0.8, 1.0];
pub static TEXT_COLOR: graphics::types::Color = [0.8, 0.8, 0.8, 1.0];

/****************************************/
/*          GAME KEYBINDINGS            */
/****************************************/

static FALL_KEYS_1P: [Key; 1] = [Key::Down];
static HARD_DROP_KEYS_1P: [Key; 1] = [Key::Space];
static RIGHT_KEYS_1P: [Key; 1] = [Key::Right];
static LEFT_KEYS_1P: [Key; 1] = [Key::Left];
static ROTATE_CLOCKWISE_KEYS_1P: [Key; 1] = [Key::Up];
static ROTATE_COUNTERCLOCKWISE_KEYS_1P: [Key; 1] = [Key::NumPad0];
static HOLD_TETROMINO_KEYS_1P: [Key; 1] = [Key::C];

static FALL_KEYS_2P: [Key; 2] = [Key::S, Key::NumPad5];
static HARD_DROP_KEYS_2P: [Key; 2] = [Key::LCtrl, Key::NumPad0];
static RIGHT_KEYS_2P: [Key; 2] = [Key::D, Key::NumPad6];
static LEFT_KEYS_2P: [Key; 2] = [Key::A, Key::NumPad4];
static ROTATE_CLOCKWISE_KEYS_2P: [Key; 2] = [Key::W, Key::NumPad8];
static ROTATE_COUNTERCLOCKWISE_KEYS_2P: [Key; 2] = [Key::E, Key::NumPad9];
static HOLD_TETROMINO_KEYS_2P: [Key; 2] = [Key::C, Key::NumPadPlus];

pub struct Keybindings {
    pub fall_keys: Vec<Key>,
    pub hard_drop_keys: Vec<Key>,
    pub right_keys: Vec<Key>,
    pub left_keys: Vec<Key>,
    pub rotate_clockwise_keys: Vec<Key>,
    pub rotate_counterclockwise_keys: Vec<Key>,
    pub hold_tetromino_keys: Vec<Key>,
}

impl Keybindings {
    pub fn new() -> Keybindings {
        let fall_keys = FALL_KEYS_1P.to_vec();
        let hard_drop_keys = HARD_DROP_KEYS_1P.to_vec();
        let right_keys = RIGHT_KEYS_1P.to_vec();
        let left_keys = LEFT_KEYS_1P.to_vec();
        let rotate_clockwise_keys = ROTATE_CLOCKWISE_KEYS_1P.to_vec();
        let rotate_counterclockwise_keys = ROTATE_COUNTERCLOCKWISE_KEYS_1P.to_vec();
        let hold_tetromino_keys = HOLD_TETROMINO_KEYS_1P.to_vec();

        Keybindings {
            fall_keys,
            hard_drop_keys,
            right_keys,
            left_keys,
            rotate_clockwise_keys,
            rotate_counterclockwise_keys,
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
            let hold_tetromino_keys = vec![HOLD_TETROMINO_KEYS_2P[0]];

            Keybindings {
                fall_keys,
                hard_drop_keys,
                right_keys,
                left_keys,
                rotate_clockwise_keys,
                rotate_counterclockwise_keys,
                hold_tetromino_keys,
            }
        } else {
            let fall_keys = vec![FALL_KEYS_2P[1]];
            let hard_drop_keys = vec![HARD_DROP_KEYS_2P[1]];
            let right_keys = vec![RIGHT_KEYS_2P[1]];
            let left_keys = vec![LEFT_KEYS_2P[1]];
            let rotate_clockwise_keys = vec![ROTATE_CLOCKWISE_KEYS_2P[1]];
            let rotate_counterclockwise_keys = vec![ROTATE_COUNTERCLOCKWISE_KEYS_2P[1]];
            let hold_tetromino_keys = vec![HOLD_TETROMINO_KEYS_2P[1]];

            Keybindings {
                fall_keys,
                hard_drop_keys,
                right_keys,
                left_keys,
                rotate_clockwise_keys,
                rotate_counterclockwise_keys,
                hold_tetromino_keys,
            }
        }
    }

    pub fn set_keys(&mut self, key_type: &TetrisCommand, new_keys: Vec<Key>) {
        match key_type {
            TetrisCommand::Fall(_) => self.fall_keys = new_keys,
            TetrisCommand::HardDrop(_) => self.hard_drop_keys = new_keys,
            TetrisCommand::Right(_) => self.right_keys = new_keys,
            TetrisCommand::Left(_) => self.left_keys = new_keys,
            TetrisCommand::RotateClockwise(_) => self.rotate_clockwise_keys = new_keys,
            TetrisCommand::RotateCounterclockwise(_) => {
                self.rotate_counterclockwise_keys = new_keys
            }
            TetrisCommand::HoldTetromino(_) => self.hold_tetromino_keys = new_keys,
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

/****************************************/
/*           APP PARAMETERS             */
/****************************************/

// not setable in the UI
pub static RESTART_KEYS: [Key; 1] = [Key::R];
pub static PAUSE_KEYS: [Key; 1] = [Key::P];
pub static KEY_REPEAT_DELAY: u64 = 20;

/// Settings represents parameters that need to be common between players in multiplayer mode.
///
/// To this purpose, it has a send() method.
#[derive(Deserialize, Debug, PartialEq)]
pub struct Settings {
    pub seed: u64,
    pub bag_size: u32,
    pub nb_next_tetromino: usize,
    remote_ip: Option<String>,
    /// Flag not to be modified except in Serialize. Set to true.
    pub serialize_as_msg: RefCell<bool>,
}

impl Settings {
    pub fn new(seed: u64, player_config: &PlayerConfig) -> Settings {
        let bag_size = BAG_SIZE;
        let nb_next_tetromino = NB_NEXT_TETROMINO;
        let mut remote_ip = None;
        if let PlayerConfig::TwoRemote {
            local_ip: _,
            remote_ip: ip,
        } = &player_config
        {
            remote_ip = Some(String::from(ip.as_str()))
        }

        Settings {
            seed,
            bag_size,
            nb_next_tetromino,
            remote_ip,
            serialize_as_msg: true.into(),
        }
    }

    pub fn set_player_config(&mut self, player_config: &PlayerConfig) {
        if let PlayerConfig::TwoRemote {
            local_ip: _,
            remote_ip: ip,
        } = &player_config
        {
            self.remote_ip = Some(String::from(ip.as_str()))
        }
    }

    /// Sends serialized settings to the remote. Should never be called when there's no remote.
    pub fn send(&self) {
        /* serialized_as_msg absolutely needs to be set to true
         * as it is used as a flag during the serialization
         * intuitively, Settings need to be serialized twice :
         * first as the SettingsMsg enum variant
         * then as the actual Settings struct
         */
        if self.remote_ip.is_none() {
            unreachable!()
        }
        if let Ok(stream) = TcpStream::connect(self.remote_ip.as_ref().unwrap()) {
            serde_cbor::to_writer::<TcpStream, Settings>(stream, self).unwrap();
        }
        once!("sent serialized settings to remote");
    }
}

pub static BAG_SIZE: u32 = 14;
// typical sizes are 7 and 14, 1 is entirely random
// for size 7 * n + k, k < 7, there's n or n + 1 of each tetromino and exactly k tetrominos are present n + 1 times
pub const NB_NEXT_TETROMINO: usize = 6;
