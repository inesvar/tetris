use super::ScoredAction;

/// Commands received by [TetrisPlayer](super::TetrisPlayer).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum TetrisCommand {
    Right,
    Left,
    Fall,
    HardDrop,
    Clockwise,
    Counterclockwise,
    HalfTurn,
    /// Put the **Tetromino in Play** in the **Hold Queue**
    /// (a new tetromino will automatically be moved to its starting position).
    Hold,
}

impl TetrisCommand {
    /// All [TetrisCommand]s.
    pub const ALL: [TetrisCommand; 8] = [
        TetrisCommand::Right,
        TetrisCommand::Left,
        TetrisCommand::Fall,
        TetrisCommand::Clockwise,
        TetrisCommand::Counterclockwise,
        TetrisCommand::HalfTurn,
        TetrisCommand::HardDrop,
        TetrisCommand::Hold,
    ];

    const HAS_AUTO_REPEAT: [TetrisCommand; 3] = [
        TetrisCommand::Right,
        TetrisCommand::Left,
        TetrisCommand::Fall,
    ];

    /// Whether the [TetrisCommand] should be repeated on a long key press.
    pub fn has_auto_repeat(&self) -> bool {
        Self::HAS_AUTO_REPEAT.contains(self)
    }
}

impl ScoredAction for TetrisCommand {
    fn score(&self) -> u32 {
        match self {
            // NOTE: at the moment we don't know how much lines a hard drop did...
            // maybe it is not TetrisCommand that should be scored ?
            TetrisCommand::Fall => 1,
            _ => 0,
        }
    }
}
