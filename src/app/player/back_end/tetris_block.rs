//! Define `struct` [Block].
use super::{
    ApplyRotationTranslation, BackendError, Deserialize, Position, RotationTranslation, Serialize,
    TetrisColor, TetrisGrid,
};

/// Block in a discrete grid, serializable.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(super) struct Block {
    pub(super) center: Position,
    color: TetrisColor,
}

impl Block {
    /// Try to apply a [RotationTranslation] in a [TetrisGrid].
    /// Move `self` by `movement` and return whether it's valid on `grid`.
    pub(super) fn try_move(
        &mut self,
        grid: &TetrisGrid,
        movement: &RotationTranslation,
        is_rotation_center_on_block_center: bool,
    ) -> Result<(), BackendError> {
        if is_rotation_center_on_block_center {
            self.center.move_by(movement);
        } else {
            self.center.move_by_with_offset(movement);
        }
        // Check if `copy` is inside `grid` and on an empty slot.
        grid.is_block_available(self)
    }
}

impl Block {
    pub(super) const fn from(color: TetrisColor, position: Position) -> Self {
        Block {
            center: position,
            color,
        }
    }

    // Is this really useful ?
    pub(super) fn x(&self) -> i32 {
        self.center.x
    }

    pub(super) fn y(&self) -> i32 {
        self.center.y
    }

    pub(super) fn color(&self) -> TetrisColor {
        self.color
    }
}

// Needed to use a circular buffer.
impl Default for Block {
    fn default() -> Self {
        Block {
            center: Position::default(),
            color: TetrisColor::Yellow, // arbitrary
        }
    }
}
