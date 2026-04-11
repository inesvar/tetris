use render_tetris::SCALE_FACTOR;

// window
pub const DEFAULT_WINDOW_WIDTH: f64 = 650.0 * SCALE_FACTOR;
pub const DEFAULT_WINDOW_HEIGHT: f64 = 800.0 * SCALE_FACTOR;

// text
pub const DEFAULT_FONT_SIZE: u32 = (16.0 * SCALE_FACTOR) as u32;

// buttons
pub const DEFAULT_BUTTON_WIDTH: f64 = 300.0 * SCALE_FACTOR;
pub const DEFAULT_BUTTON_HEIGHT: f64 = 50.0 * SCALE_FACTOR;
pub const DEFAULT_BUTTON_Y_SPACING: f64 = 100.0 * SCALE_FACTOR;

// key inputs
pub const DEFAULT_KEY_INPUT_WIDTH: f64 = 200.0 * SCALE_FACTOR;
pub const DEFAULT_KEY_INPUT_HEIGHT: f64 = 50.0 * SCALE_FACTOR;

// cursor
pub const CURSOR_BLINK_PERIOD: f64 = 1.0;

// TODO : shouldn't be here !
pub const HOST_PORT: &str = ":26000";
pub const GUEST_PORT: &str = ":26005";
