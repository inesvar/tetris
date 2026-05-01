use crate::TetrominoKind;
use serde::{Deserialize, Serialize};
use std::fmt;

/// 7 [Tetromino](crate::Tetromino) colors and 1 garbage color.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
#[allow(missing_docs)]
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
            Self::Cyan => "I",
            Self::Yellow => "O",
            Self::Purple => "T",
            Self::Blue => "L",
            Self::Orange => "J",
            Self::Green => "S",
            Self::Red => "Z",
            Self::Grey => "X",
        };
        write!(f, "{str}")
    }
}

impl TryFrom<char> for TetrisColor {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'I' => Ok(Self::Cyan),
            'O' => Ok(Self::Yellow),
            'T' => Ok(Self::Purple),
            'L' => Ok(Self::Blue),
            'J' => Ok(Self::Orange),
            'S' => Ok(Self::Green),
            'Z' => Ok(Self::Red),
            'X' => Ok(Self::Grey),
            _ => Err(()),
        }
    }
}

impl TetrisColor {
    /// Get lowercase name.
    pub fn get_lowercase_name(&self) -> &str {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::o(TetrominoKind::O, TetrisColor::Yellow)]
    #[case::i(TetrominoKind::I, TetrisColor::Cyan)]
    #[case::t(TetrominoKind::T, TetrisColor::Purple)]
    #[case::l(TetrominoKind::L, TetrisColor::Orange)]
    #[case::j(TetrominoKind::J, TetrisColor::Blue)]
    #[case::s(TetrominoKind::S, TetrisColor::Green)]
    #[case::z(TetrominoKind::Z, TetrisColor::Red)]
    fn tetris_color_from_tetromino_kind_is_correct(
        #[case] kind: TetrominoKind,
        #[case] color: TetrisColor,
    ) {
        assert_eq!(TetrisColor::from(kind), color);
    }
}
