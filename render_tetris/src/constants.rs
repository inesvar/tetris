pub const SCALE_FACTOR: f64 = 1.0;

// coordinates of the top left corner of the grid
// changing this moves everything in the single-player view except the title
pub const DEFAULT_GRID_X: f64 = 200.0 * SCALE_FACTOR;
pub const DEFAULT_GRID_Y: f64 = 160.0 * SCALE_FACTOR;

pub const GRID_BG_COLOR: graphics::types::Color = [0.3, 0.3, 0.3, 1.0];
pub const GRID_COLOR: graphics::types::Color = [0.8, 0.8, 0.8, 1.0];

pub const BLOCK_SIZE: f64 = 25.0 * SCALE_FACTOR;
pub const TETROMINO_MAX_WIDTH: f64 = 4.0 * BLOCK_SIZE;
pub const TETROMINO_MAX_HEIGHT: f64 = 2.0 * BLOCK_SIZE;
pub const GRID_THICKNESS: f64 = 0.5;
