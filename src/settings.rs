use graphics::color;
use opengl_graphics::OpenGL;
use piston::Key;

// Change this to OpenGL::V2_1 if not working.
pub static OPENGL_VERSION: OpenGL = OpenGL::V4_5;

pub static NB_COLUMNS: u32 = 10;
pub static NB_ROWS: u32 = 22;

pub static DEFAULT_WINDOW_WIDTH: u32 = 650;
pub static DEFAULT_WINDOW_HEIGHT: u32 = 700;

pub static BLOCK_SIZE: f64 = 25.0;
pub static GRID_THICKNESS: f64 = 0.5;

pub static BG_COLOR: graphics::types::Color = [0.0, 0.0, 0.2, 1.0];
pub static GRID_BG_COLOR: graphics::types::Color = [0.1, 0.1, 0.1, 1.0];
pub static GRID_COLOR: graphics::types::Color = color::GRAY;

pub static FALL_KEYS: [Key; 2] = [Key::Down, Key::S];
pub static HARD_DROP_KEYS: [Key; 1] = [Key::Space];
pub static RIGHT_KEYS: [Key; 2] = [Key::Right, Key::D];
pub static LEFT_KEYS: [Key; 2] = [Key::Left, Key::Q];
pub static ROTATE_CLOCKWISE_KEYS: [Key; 2] = [Key::Up, Key::E];
pub static ROTATE_COUNTERCLOCKWISE_KEYS: [Key; 2] = [Key::NumPad3, Key::A];
pub static RESTART_KEYS: [Key; 1] = [Key::R];
pub static HOLD_TETROMINO_KEYS: [Key; 1] = [Key::C];

pub static KEY_REPEAT_DELAY: u64 = 20;

pub static BAG_SIZE: u32 = 14; // typical sizes are 7 and 14, 1 is entirely random
                               // for size 7 * n + k, there's n or n + 1 of each tetromino and exactly k tetrominos are present n + 1 times

pub const NB_NEXT_TETROMINO: usize = 6;

pub static SERVER_IP: &str = "127.0.0.1:16000";
pub static VIEWER_IP: &str = "127.0.0.1:16001";
//IMPORTANT: do not use localhost, only use the result of hostname -I

pub static SEED: u64 = 64;
