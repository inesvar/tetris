//! Settings include sizes and colors in the UI and default keybindings.
use core_tetris::BagType;
use opengl_graphics::OpenGL;
use piston::Key;
use render_tetris::{BLOCK_SIZE, DEFAULT_GRID_Y, SCALE_FACTOR};
use serde::{Deserialize, Serialize};

// Change this to OpenGL::V2_1 if not working.
pub const OPENGL_VERSION: OpenGL = OpenGL::V4_5;

// height of the title "Tetris" and "Press R to (re)start"
pub const DEFAULT_TITLE_Y: f64 = 100.0 * SCALE_FACTOR;
// height of the text on the left side indicating the score
pub const DEFAULT_SCORE_TEXT_Y: f64 = DEFAULT_GRID_Y + 8.0 * BLOCK_SIZE;

/****************************************/
/*               COLORS                 */
/****************************************/

pub const BG_COLOR: graphics::types::Color = [0.1, 0.1, 0.1, 1.0];

/****************************************/
/*           APP PARAMETERS             */
/****************************************/

pub const FALL_SPEED_DIVIDE: u64 = 50;
pub const FREEZE: u64 = 50;
// not setable in the UI
pub const RESTART_KEYS: [Key; 1] = [Key::R];
pub const PAUSE_KEYS: [Key; 1] = [Key::P];
pub const AUTO_REPEAT_DELAY: u64 = 20; // should be around 0.3s (36) according to the Guideline 5.2 Auto-repeat
pub const AUTO_REPEAT_SPEED: u64 = 3; // should be around 25ms (3) according to the Guideline 5.2 Auto-repeat

/// Settings represents parameters that need to be common between players in multiplayer mode.
///
/// To this purpose, it has a send() method.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Settings {
    pub seed: u64,
    pub bag_size: BagType,
    pub nb_next_tetromino: usize,
}

impl Settings {
    pub fn new(seed: u64) -> Settings {
        let bag_size = BAG_TYPE;
        let nb_next_tetromino = NB_NEXT_TETROMINO;

        Settings {
            seed,
            bag_size,
            nb_next_tetromino,
        }
    }
}

pub const BAG_TYPE: BagType = BagType::Bag7;
// Should be between 0 and 6
pub const NB_NEXT_TETROMINO: usize = 6;
