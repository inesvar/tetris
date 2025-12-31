#![allow(unused)]
use super::rotation_translation::RotationType;
use super::spatial_primitives::{Position, FALL, RIGHT, LEFT};

#[derive(PartialEq)]
pub(in crate::app::player) enum TetrominoMove {
    Right,
    Left,
    Fall,
    HardDrop,
    Clockwise,
    Counterclockwise,
    HalfTurn,
}

impl TetrominoMove {
    pub(super) fn is_repeated(&self) -> bool {
        *self == Self::HardDrop
    }

    pub(super) fn has_wall_kicks(&self) -> bool {
        *self == Self::Clockwise || *self == Self::Counterclockwise
    }
}

impl From<&TetrominoMove> for Position {
    fn from(value: &TetrominoMove) -> Self {
        match value {
            TetrominoMove::Fall | TetrominoMove::HardDrop => FALL,
            TetrominoMove::Right => RIGHT,
            TetrominoMove::Left => LEFT,
            _ => Self::default(),
        }
    }
}

impl From<&TetrominoMove> for RotationType {
    fn from(value: &TetrominoMove) -> Self {
        match value {
            TetrominoMove::Clockwise => Self::Clockwise,
            TetrominoMove::Counterclockwise => Self::Counterclockwise,
            TetrominoMove::HalfTurn => Self::HalfTurn,
            _ => Self::None,
        }
    }
}
