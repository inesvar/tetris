//! Define `enum` [TetrominoMove].
#![doc = simple_mermaid::mermaid!("tetromino_move.mmd")]

use super::{Position, RotationType};
use crate::TetrisCommand;

/// Movements of the [Tetromino](super::Tetromino) in the [TetrisGrid](super::TetrisGrid).
pub(crate) struct TetrominoMove {
    is_repeated: bool,
    translation: Position,
    rotation: RotationType,
}

impl TetrominoMove {
    pub(super) fn rotation(rotation: RotationType) -> Self {
        Self {
            is_repeated: false,
            translation: Position::ZERO,
            rotation,
        }
    }

    pub(super) fn translation(translation: Position) -> Self {
        Self {
            is_repeated: false,
            translation,
            rotation: RotationType::Identity,
        }
    }

    pub(super) fn repeat(translation: Position) -> Self {
        Self {
            is_repeated: true,
            translation,
            rotation: RotationType::Identity,
        }
    }

    pub(super) fn is_repeated(&self) -> bool {
        self.is_repeated
    }

    pub(super) fn get_translation(&self) -> Position {
        self.translation
    }

    pub(super) fn get_rotation_type(&self) -> RotationType {
        self.rotation
    }
}

impl From<TetrisCommand> for TetrominoMove {
    fn from(value: TetrisCommand) -> Self {
        match value {
            TetrisCommand::Right => Self::translation(Position::RIGHT),
            TetrisCommand::Left => Self::translation(Position::LEFT),
            TetrisCommand::Fall => Self::translation(Position::FALL),
            TetrisCommand::HardDrop => Self::repeat(Position::FALL),
            TetrisCommand::Clockwise => Self::rotation(RotationType::Clockwise),
            TetrisCommand::Counterclockwise => Self::rotation(RotationType::Counterclockwise),
            TetrisCommand::HalfTurn => Self::rotation(RotationType::HalfTurn),
            TetrisCommand::Hold => Self::rotation(RotationType::Identity),
        }
    }
}

impl From<RotationType> for TetrominoMove {
    fn from(value: RotationType) -> Self {
        TetrominoMove::rotation(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::fall(TetrisCommand::Fall, Position::FALL, RotationType::Identity, false)]
    #[case::right(TetrisCommand::Right, Position::RIGHT, RotationType::Identity, false)]
    #[case::left(TetrisCommand::Left, Position::LEFT, RotationType::Identity, false)]
    #[case::hard_drop(TetrisCommand::HardDrop, Position::FALL, RotationType::Identity, true)]
    #[case::clockwise(
        TetrisCommand::Clockwise,
        Position::default(),
        RotationType::Clockwise,
        false
    )]
    #[case::counterclockwise(
        TetrisCommand::Counterclockwise,
        Position::default(),
        RotationType::Counterclockwise,
        false
    )]
    #[case::half_turn(
        TetrisCommand::HalfTurn,
        Position::default(),
        RotationType::HalfTurn,
        false
    )]
    fn tetromino_move_decomposition_is_correct(
        #[case] command: TetrisCommand,
        #[case] translation: Position,
        #[case] rotation: RotationType,
        #[case] is_repeated: bool,
    ) {
        let movement = TetrominoMove::from(command);
        assert_eq!(movement.get_translation(), translation);
        assert_eq!(movement.get_rotation_type(), rotation);
        assert_eq!(movement.is_repeated(), is_repeated);
    }
}
