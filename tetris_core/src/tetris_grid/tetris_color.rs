use crate::TetrominoKind;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum TetrisColor {
    Cyan,
    Yellow,
    Purple,
    Blue,
    Orange,
    Green,
    Red,
    #[default]
    /// Default.
    Grey,
}

impl fmt::Display for TetrisColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Cyan => "C",
            Self::Yellow => "Y",
            Self::Purple => "P",
            Self::Blue => "B",
            Self::Orange => "O",
            Self::Green => "G",
            Self::Red => "R",
            Self::Grey => "X",
        };
        write!(f, "{str}")
    }
}

impl TryFrom<char> for TetrisColor {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'C' => Ok(Self::Cyan),
            'Y' => Ok(Self::Yellow),
            'P' => Ok(Self::Purple),
            'B' => Ok(Self::Blue),
            'O' => Ok(Self::Orange),
            'G' => Ok(Self::Green),
            'R' => Ok(Self::Red),
            'X' => Ok(Self::Grey),
            _ => Err(()),
        }
    }
}

impl TetrisColor {
    pub fn get_texture_filename(&self) -> &str {
        match self {
            Self::Cyan => "cyan",
            Self::Yellow => "yellow",
            Self::Purple => "purple",
            Self::Blue => "blue",
            Self::Orange => "orange",
            Self::Green => "green",
            Self::Red => "red",
            Self::Grey => "grey",
        }
    }
}

impl From<TetrominoKind> for TetrisColor {
    fn from(kind: TetrominoKind) -> Self {
        match kind {
            TetrominoKind::I => TetrisColor::Cyan,
            TetrominoKind::O => TetrisColor::Yellow,
            TetrominoKind::T => TetrisColor::Purple,
            TetrominoKind::J => TetrisColor::Blue,
            TetrominoKind::L => TetrisColor::Orange,
            TetrominoKind::S => TetrisColor::Green,
            TetrominoKind::Z => TetrisColor::Red,
        }
    }
}
