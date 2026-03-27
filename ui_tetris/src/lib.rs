//! User interface components of the executable.
mod button;
mod constants;
pub mod interactive_widget_manager;
mod key_input;
mod keybindings;
mod render;
mod text;
mod text_input;

pub(crate) use constants::*;

pub use constants::{
    DEFAULT_FONT_SIZE, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, GUEST_PORT, HOST_PORT,
    TEXT_COLOR,
};
pub use keybindings::{KeyLookup, Keybindings};
pub use render::RenderTetrisUi;
pub use text::Text;
