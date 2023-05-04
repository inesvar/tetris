use graphics::color;
use piston::Key;

pub static DEFAULT_WINDOW_WIDTH: u32 = 500;
pub static DEFAULT_WINDOW_HEIGHT: u32 = 700;

pub static BLOCK_SIZE: f64 = 25.0;
pub static GRID_THICKNESS: f64 = 0.5;

pub static BG_COLOR: graphics::types::Color = color::BLACK;
pub static GRID_COLOR: graphics::types::Color = color::GRAY;

pub static FALL_KEYS: [Key; 2] = [Key::Down, Key::C];
pub static HARD_DROP_KEYS: [Key; 1] = [Key::Space];
pub static RIGHT_KEYS: [Key; 2] = [Key::Right, Key::V];
pub static LEFT_KEYS: [Key; 2] = [Key::Left, Key::X];
pub static ROTATE_CLOCKWISE_KEYS: [Key; 2] = [Key::Up, Key::F];
pub static ROTATE_COUNTERCLOCKWISE_KEYS: [Key; 2] = [Key::NumPad3, Key::D];