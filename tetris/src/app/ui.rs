//! User interface components of the executable.
pub mod button;
pub mod interactive_widget_manager;
pub mod key_input;
pub mod render;
pub mod text;
pub mod text_input;

pub use interactive_widget_manager::TetrisCommand;
pub use render::{render_text, render_widget_manager};
