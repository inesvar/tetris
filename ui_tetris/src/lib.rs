//! User interface components of the executable.
pub mod button;
mod constants;
pub mod interactive_widget_manager;
pub mod key_input;
mod keybindings;
pub mod render;
pub mod text;
pub mod text_input;

pub(crate) use constants::*;
pub use constants::{
    DEFAULT_FONT_SIZE, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, GUEST_PORT, HOST_PORT,
    TEXT_COLOR,
};
pub use interactive_widget_manager::TetrisCommand;
pub use keybindings::Keybindings;
pub use render::RenderTetrisUi;
