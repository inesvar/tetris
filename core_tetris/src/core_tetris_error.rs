use super::GameOverError;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
#[allow(missing_docs)]
/// Error type for [core_tetris](crate).
pub enum CoreTetrisError {
    EmptyCircularBuffer,
    InvalidTetrisPlayer(GameOverError),
}

impl Error for CoreTetrisError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidTetrisPlayer(err) => Some(err),
            _ => None,
        }
    }
}

impl Display for CoreTetrisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyCircularBuffer => write!(f, "unable to create empty CircularBuffer"),
            Self::InvalidTetrisPlayer(error) => write!(
                f,
                "unable to create TetrisPlayer in a game over situation : {error}"
            ),
        }
    }
}

impl From<GameOverError> for CoreTetrisError {
    fn from(value: GameOverError) -> Self {
        Self::InvalidTetrisPlayer(value)
    }
}
