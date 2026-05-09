use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
#[allow(missing_docs)]
/// Error type for [core_tetris](crate).
pub enum CoreTetrisError {
    EmptyCircularBuffer,
}

impl Error for CoreTetrisError {}

impl Display for CoreTetrisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyCircularBuffer => write!(f, "unable to create empty CircularBuffer"),
        }
    }
}
