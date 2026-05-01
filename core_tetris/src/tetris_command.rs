use super::TetrominoMove;

/// Commands received by [TetrisPlayer](super::TetrisPlayer).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TetrisCommand {
    /// Move the **Tetromino in Play**.
    Move(TetrominoMove),
    /// Put the **Tetromino in Play** in the **Hold Queue**
    /// (a new tetromino will automatically be moved to its starting position).
    Hold,
}

impl TetrisCommand {
    /// All [TetrisCommand]s.
    pub const ALL: [TetrisCommand; 8] = [
        TetrisCommand::Move(TetrominoMove::Right),
        TetrisCommand::Move(TetrominoMove::Left),
        TetrisCommand::Move(TetrominoMove::Fall),
        TetrisCommand::Move(TetrominoMove::Clockwise),
        TetrisCommand::Move(TetrominoMove::Counterclockwise),
        TetrisCommand::Move(TetrominoMove::HalfTurn),
        TetrisCommand::Move(TetrominoMove::HardDrop),
        TetrisCommand::Hold,
    ];

    const HAS_AUTO_REPEAT: [TetrisCommand; 3] = [
        TetrisCommand::Move(TetrominoMove::Right),
        TetrisCommand::Move(TetrominoMove::Left),
        TetrisCommand::Move(TetrominoMove::Fall),
    ];

    /// Whether the [TetrisCommand] should be repeated on a long key press.
    pub fn has_auto_repeat(&self) -> bool {
        Self::HAS_AUTO_REPEAT.contains(self)
    }
}

impl From<TetrominoMove> for TetrisCommand {
    fn from(value: TetrominoMove) -> Self {
        TetrisCommand::Move(value)
    }
}
