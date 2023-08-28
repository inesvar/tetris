use std::{cell::RefCell, net::TcpStream};

use crate::{once, ui::interactive_widget_manager::KeyInputType, PlayerConfig};
use opengl_graphics::OpenGL;
use piston::Key;
use serde::Deserialize;

static SCALE_FACTOR: f64 = 0.7;

// Change this to OpenGL::V2_1 if not working.
pub static OPENGL_VERSION: OpenGL = OpenGL::V4_5;

pub static NB_COLUMNS: u32 = 10;
pub static NB_ROWS: u32 = 22;

/****************************************/
/* POSITIONS IN SINGLE PLAYER GAME VIEW */
/****************************************/

pub static DEFAULT_WINDOW_WIDTH: u32 = (650.0 * SCALE_FACTOR) as u32;
pub static DEFAULT_WINDOW_HEIGHT: u32 = (700.0 * SCALE_FACTOR) as u32;

// coordinates of the top left corner of the grid
// changing this moves everything in the single-player view except the title
pub static DEFAULT_GRID_X: f64 = 200.0 * SCALE_FACTOR;
pub static DEFAULT_GRID_Y: f64 = 120.0 * SCALE_FACTOR;

// height of the title "Tetris" and "Press R to (re)start"
pub static DEFAULT_TITLE_Y: f64 = 50.0 * SCALE_FACTOR;
// height of the text on the left side indicating the score
pub static DEFAULT_SCORE_TEXT_Y: f64 = DEFAULT_GRID_Y + 8.0 * BLOCK_SIZE;

pub static BLOCK_SIZE: f64 = 25.0 * SCALE_FACTOR;
pub static TETROMINO_MAX_WIDTH: f64 = 4.0 * BLOCK_SIZE;
pub static TETROMINO_MAX_HEIGHT: f64 = 2.0 * BLOCK_SIZE;
pub static GRID_THICKNESS: f64 = 0.5 * SCALE_FACTOR;

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
        let fall_keys = FALL_KEYS.to_vec();
        let hard_drop_keys = HARD_DROP_KEYS.to_vec();
        let right_keys = RIGHT_KEYS.to_vec();
        let left_keys = LEFT_KEYS.to_vec();
        let rotate_clockwise_keys = ROTATE_CLOCKWISE_KEYS.to_vec();
        let rotate_counterclockwise_keys = ROTATE_COUNTERCLOCKWISE_KEYS.to_vec();
        let hold_tetromino_keys = HOLD_TETROMINO_KEYS.to_vec();

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

    pub fn set_keys(&mut self, key_type: &KeyInputType, new_keys: Vec<Key>) {
        match key_type {
            KeyInputType::FallKey(_) => self.fall_keys = new_keys,
            KeyInputType::HardDropKey(_) => self.hard_drop_keys = new_keys,
            KeyInputType::RightKey(_) => self.right_keys = new_keys,
            KeyInputType::LeftKey(_) => self.left_keys = new_keys,
            KeyInputType::RotateClockwiseKey(_) => self.rotate_clockwise_keys = new_keys,
            KeyInputType::RotateCounterclockwiseKey(_) => {
                self.rotate_counterclockwise_keys = new_keys
            }
            KeyInputType::HoldTetrominoKey(_) => self.hold_tetromino_keys = new_keys,
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

static FALL_KEYS: [Key; 2] = [Key::Down, Key::S];
static HARD_DROP_KEYS: [Key; 1] = [Key::Space];
static RIGHT_KEYS: [Key; 2] = [Key::Right, Key::D];
static LEFT_KEYS: [Key; 2] = [Key::Left, Key::Q];
static ROTATE_CLOCKWISE_KEYS: [Key; 2] = [Key::Up, Key::E];
static ROTATE_COUNTERCLOCKWISE_KEYS: [Key; 2] = [Key::NumPad3, Key::A];
static HOLD_TETROMINO_KEYS: [Key; 1] = [Key::C];

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
        match &player_config {
            PlayerConfig::Streamer(ip) => remote_ip = Some(String::from(ip.as_str())),
            PlayerConfig::TwoRemote {
                local_ip: _,
                remote_ip: ip,
            } => remote_ip = Some(String::from(ip.as_str())),
            _ => {}
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
        match &player_config {
            PlayerConfig::Streamer(ip) => self.remote_ip = Some(String::from(ip.as_str())),
            PlayerConfig::TwoRemote {
                local_ip: _,
                remote_ip: ip,
            } => self.remote_ip = Some(String::from(ip.as_str())),
            _ => {}
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
        match self.remote_ip {
            None => unreachable!(),
            _ => {}
        }
        if let Ok(stream) = TcpStream::connect(self.remote_ip.as_ref().unwrap()) {
            serde_cbor::to_writer::<TcpStream, Settings>(stream, &self).unwrap();
        }
        once!("sent serialized data to remote");
    }
}

pub static BAG_SIZE: u32 = 14; // typical sizes are 7 and 14, 1 is entirely random
                               // for size 7 * n + k, k < 7, there's n or n + 1 of each tetromino and exactly k tetrominos are present n + 1 times
pub const NB_NEXT_TETROMINO: usize = 6;
