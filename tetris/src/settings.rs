//! Settings include sizes and colors in the UI and default keybindings.

use std::{cell::RefCell, net::TcpStream};

use crate::{app::PlayerConfig, once};
use core_tetris::BagType;
use opengl_graphics::OpenGL;
use piston::Key;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

const SCALE_FACTOR: f64 = 1.0;
pub const HOST_PORT: &str = ":26000";
pub const GUEST_PORT: &str = ":26005";

// Change this to OpenGL::V2_1 if not working.
pub const OPENGL_VERSION: OpenGL = OpenGL::V4_5;

/****************************************/
/* POSITIONS IN SINGLE PLAYER GAME VIEW */
/****************************************/

pub const DEFAULT_WINDOW_WIDTH: u32 = (650.0 * SCALE_FACTOR) as u32;
pub const DEFAULT_WINDOW_HEIGHT: u32 = (800.0 * SCALE_FACTOR) as u32;

// coordinates of the top left corner of the grid
// changing this moves everything in the single-player view except the title
pub const DEFAULT_GRID_X: f64 = 200.0 * SCALE_FACTOR;
pub const DEFAULT_GRID_Y: f64 = 160.0 * SCALE_FACTOR;

// height of the title "Tetris" and "Press R to (re)start"
pub const DEFAULT_TITLE_Y: f64 = 100.0 * SCALE_FACTOR;
// height of the text on the left side indicating the score
pub const DEFAULT_SCORE_TEXT_Y: f64 = DEFAULT_GRID_Y + 8.0 * BLOCK_SIZE;

pub const BLOCK_SIZE: f64 = 25.0 * SCALE_FACTOR;
pub const TETROMINO_MAX_WIDTH: f64 = 4.0 * BLOCK_SIZE;
pub const TETROMINO_MAX_HEIGHT: f64 = 2.0 * BLOCK_SIZE;
pub const GRID_THICKNESS: f64 = 0.5;

/****************************************/
/*      POSITIONS IN MAIN MENU VIEW     */
/****************************************/

// size of the buttons
pub const DEFAULT_BUTTON_WIDTH: f64 = 300.0 * SCALE_FACTOR;
pub const DEFAULT_BUTTON_HEIGHT: f64 = 50.0 * SCALE_FACTOR;
pub const DEFAULT_BUTTON_Y_SPACING: f64 = 100.0 * SCALE_FACTOR;
// size of the text
pub const DEFAULT_FONT_SIZE: u32 = (16.0 * SCALE_FACTOR) as u32;

/****************************************/
/*      POSITIONS IN SETTINGS VIEW      */
/****************************************/

// size of the buttons
pub const DEFAULT_KEY_INPUT_WIDTH: f64 = 200.0 * SCALE_FACTOR;
pub const DEFAULT_KEY_INPUT_HEIGHT: f64 = 50.0 * SCALE_FACTOR;

/****************************************/
/*               COLORS                 */
/****************************************/

pub const BG_COLOR: graphics::types::Color = [0.1, 0.1, 0.1, 1.0];
pub const GRID_BG_COLOR: graphics::types::Color = [0.3, 0.3, 0.3, 1.0];
pub const GRID_COLOR: graphics::types::Color = [0.8, 0.8, 0.8, 1.0];
pub const TEXT_COLOR: graphics::types::Color = [0.8, 0.8, 0.8, 1.0];

/****************************************/
/*             ANIMATIONS               */
/****************************************/

pub const CURSOR_BLINK_PERIOD: f64 = 1.0;

/****************************************/
/*           APP PARAMETERS             */
/****************************************/

pub const FALL_SPEED_DIVIDE: u64 = 50;
pub const FREEZE: u64 = 50;
// not setable in the UI
pub const RESTART_KEYS: [Key; 1] = [Key::R];
pub const PAUSE_KEYS: [Key; 1] = [Key::P];
pub const KEY_REPEAT_DELAY: u64 = 20;

/// Settings represents parameters that need to be common between players in multiplayer mode.
///
/// To this purpose, it has a send() method.
#[derive(Deserialize, Debug, PartialEq)]
pub struct Settings {
    pub seed: u64,
    pub bag_size: BagType,
    pub nb_next_tetromino: usize,
    remote_ip: Option<String>,
    /// Flag not to be modified except in Serialize. Set to true.
    pub serialize_as_msg: RefCell<bool>,
}

impl Settings {
    pub fn new(seed: u64, player_config: &PlayerConfig) -> Settings {
        let bag_size = BAG_TYPE;
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

pub const BAG_TYPE: BagType = BagType::Bag7;
// Should be between 0 and 6
pub const NB_NEXT_TETROMINO: usize = 6;

impl Serialize for Settings {
    /// Serializes this value.
    ///
    /// It is serialized as SettingsMsg(self) if serialize_as_msg is set to true.
    /// Otherwise, it's serialized as it would with #[derive(Serialize)].
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !*self.serialize_as_msg.borrow() {
            let mut s = serializer.serialize_struct("Settings", 4)?;
            s.serialize_field("seed", &self.seed)?;
            s.serialize_field("bag_size", &self.bag_size)?;
            s.serialize_field("nb_next_tetromino", &self.nb_next_tetromino)?;
            s.serialize_field("serialize_as_msg", &self.serialize_as_msg)?;
            s.end()
        } else {
            {
                let mut a = self.serialize_as_msg.borrow_mut();
                *a = false;
            }
            let s = serializer.serialize_newtype_variant("MessageType", 1, "Settings", self);
            {
                let mut a = self.serialize_as_msg.borrow_mut();
                *a = true;
            }
            s
        }
    }
}
