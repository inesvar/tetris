//! Define `enum` [TetrominoMove].
#![doc = simple_mermaid::mermaid!("tetromino_move.mmd")]

use super::{Position, RotationType};

/// Movements of the [Tetromino](super::Tetromino) in the [TetrisGrid](super::TetrisGrid).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum TetrominoMove {
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

    pub(super) fn get_translation(&self) -> Position {
        match self {
            Self::Fall | Self::HardDrop => Position::FALL,
            Self::Right => Position::RIGHT,
            Self::Left => Position::LEFT,
            _ => Position::default(),
        }
    }

    pub(super) fn get_rotation_type(&self) -> RotationType {
        match self {
            Self::Clockwise => RotationType::Clockwise,
            Self::Counterclockwise => RotationType::Counterclockwise,
            Self::HalfTurn => RotationType::HalfTurn,
            _ => RotationType::Identity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::fall(TetrominoMove::Fall, Position::FALL, RotationType::Identity, false)]
    #[case::right(TetrominoMove::Right, Position::RIGHT, RotationType::Identity, false)]
    #[case::left(TetrominoMove::Left, Position::LEFT, RotationType::Identity, false)]
    #[case::hard_drop(TetrominoMove::HardDrop, Position::FALL, RotationType::Identity, true)]
    #[case::clockwise(
        TetrominoMove::Clockwise,
        Position::default(),
        RotationType::Clockwise,
        false
    )]
    #[case::counterclockwise(
        TetrominoMove::Counterclockwise,
        Position::default(),
        RotationType::Counterclockwise,
        false
    )]
    #[case::half_turn(
        TetrominoMove::HalfTurn,
        Position::default(),
        RotationType::HalfTurn,
        false
    )]
    fn tetromino_move_decomposition_is_correct(
        #[case] movement: TetrominoMove,
        #[case] translation: Position,
        #[case] rotation: RotationType,
        #[case] is_repeated: bool,
    ) {
        assert_eq!(movement.get_translation(), translation);
        assert_eq!(movement.get_rotation_type(), rotation);
        assert_eq!(movement.is_repeated(), is_repeated);
    }
}
