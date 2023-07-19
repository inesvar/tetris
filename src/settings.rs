use crate::ui::interactive_widget_manager::KeyInputType;
use opengl_graphics::OpenGL;
use piston::Key;

// Change this to OpenGL::V2_1 if not working.
pub static OPENGL_VERSION: OpenGL = OpenGL::V4_5;

pub static NB_COLUMNS: u32 = 10;
pub static NB_ROWS: u32 = 22;

/****************************************/
/* POSITIONS IN SINGLE PLAYER GAME VIEW */
/****************************************/

pub static DEFAULT_WINDOW_WIDTH: u32 = 650;
pub static DEFAULT_WINDOW_HEIGHT: u32 = 700;

// coordinates of the top left corner of the grid
// changing this moves everything in the single-player view except the title
pub static DEFAULT_GRID_X: f64 = 200.0;
pub static DEFAULT_GRID_Y: f64 = 120.0;

// height of the title "Tetris" and "Press R to (re)start"
pub static DEFAULT_TITLE_Y: f64 = 50.0;
// height of the text on the left side indicating the score
pub static DEFAULT_SCORE_TEXT_Y: f64 = DEFAULT_GRID_Y + 8.0 * BLOCK_SIZE;

pub static BLOCK_SIZE: f64 = 25.0;
pub static TETROMINO_MAX_WIDTH: f64 = 4.0 * BLOCK_SIZE;
pub static TETROMINO_MAX_HEIGHT: f64 = 2.0 * BLOCK_SIZE;
pub static GRID_THICKNESS: f64 = 0.5;

/****************************************/
/*      POSITIONS IN MAIN MENU VIEW     */
/****************************************/

// size of the buttons
pub static DEFAULT_BUTTON_WIDTH: f64 = 300.0;
pub static DEFAULT_BUTTON_HEIGHT: f64 = 50.0;

/****************************************/
/*      POSITIONS IN SETTINGS VIEW      */
/****************************************/

// size of the buttons
pub static DEFAULT_KEY_INPUT_WIDTH: f64 = 200.0;
pub static DEFAULT_KEY_INPUT_HEIGHT: f64 = 50.0;

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

pub struct Settings {
    pub fall_keys: Vec<Key>,
    pub hard_drop_keys: Vec<Key>,
    pub right_keys: Vec<Key>,
    pub left_keys: Vec<Key>,
    pub rotate_clockwise_keys: Vec<Key>,
    pub rotate_counterclockwise_keys: Vec<Key>,
    pub hold_tetromino_keys: Vec<Key>,
}

impl Settings {
    pub fn new() -> Settings {
        let fall_keys = FALL_KEYS.to_vec();
        let hard_drop_keys = HARD_DROP_KEYS.to_vec();
        let right_keys = RIGHT_KEYS.to_vec();
        let left_keys = LEFT_KEYS.to_vec();
        let rotate_clockwise_keys = ROTATE_CLOCKWISE_KEYS.to_vec();
        let rotate_counterclockwise_keys = ROTATE_COUNTERCLOCKWISE_KEYS.to_vec();
        let hold_tetromino_keys = HOLD_TETROMINO_KEYS.to_vec();

        Settings {
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

pub static RESTART_KEYS: [Key; 1] = [Key::R];
pub static PAUSE_KEYS: [Key; 1] = [Key::P];

pub static KEY_REPEAT_DELAY: u64 = 20;

pub static BAG_SIZE: u32 = 14; // typical sizes are 7 and 14, 1 is entirely random
                               // for size 7 * n + k, k < 7, there's n or n + 1 of each tetromino and exactly k tetrominos are present n + 1 times

pub const NB_NEXT_TETROMINO: usize = 6;

/****************************************/
/*       ONLINE GAME PARAMETERS         */
/****************************************/

pub static SERVER_IP: &str = "127.0.0.1:16000";
pub static VIEWER_IP: &str = "127.0.0.1:16001";
//IMPORTANT: do not use localhost, only use the result of hostname -I
