//! Define `struct` [Tetromino], re-export `struct` [Position] and `enum`s [TetrominoKind] and [RotationType].
#![doc = simple_mermaid::mermaid!("tetromino/tetromino.mmd")]

mod moving_primitives;
mod rotation_translation;
mod spatial_primitives;
mod tetromino_kind;
mod tetromino_move;

use self::{
    moving_primitives::ApplyRotationTranslation, rotation_translation::RotationTranslation,
    spatial_primitives::Direction,
};
use super::{GameOverError, TetrisColor, TetrisGrid};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub(super) use rotation_translation::RotationType;
pub use spatial_primitives::Position;
pub use tetromino_kind::TetrominoKind;
pub use tetromino_move::TetrominoMove;

/// Tetromino.
///
/// According to the **Tetris Guideline**, a tetromino is a "\[shape\] made from four connected squares":
///
/// > "There are seven differently shaped tetriminos that fall into a rectangular **Matrix**.
/// > As tetriminos fall, a player may rotate, move, or drop them into their final resting place.
/// > If a row or more of cells is completely filled with **Blocks**, then the line or lines are cleared from the **Matrix**"
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct Tetromino {
    kind: TetrominoKind,
    center: Position,
    pub(super) blocks: [Position; 4],
    direction: Direction,
}

impl From<TetrominoKind> for Tetromino {
    fn from(kind: TetrominoKind) -> Tetromino {
        let positions = kind.get_initial_position();
        Tetromino {
            kind,
            center: positions[0],
            blocks: [positions[1], positions[2], positions[3], positions[4]],
            direction: Direction::default(),
        }
    }
}

// #[doc = simple_mermaid::mermaid!("tetromino/tetromino_internals.mmd")]
/// Constructors and getters.
impl Tetromino {
    /// Creates a new [Tetromino] of [TetrominoKind] `kind` in the default position and orientation.
    ///
    /// # Examples
    /// ```
    /// # use tetris_core::{Tetromino, TetrominoKind};
    /// // The O Tetromino was arbitrarily chosen as the default one
    /// assert_eq!(Tetromino::new(TetrominoKind::O), Tetromino::default());
    /// ```
    pub fn new(kind: TetrominoKind) -> Self {
        Self::from(kind)
    }

    /// Resets `self` in the default position and orientation, keeping the same [TetrominoKind].
    ///
    /// # Examples
    /// ```
    /// # use tetris_core::{Tetromino, TetrominoKind, TetrisGrid, TetrominoMove};
    /// # let mut o_tetromino = Tetromino::default();
    /// # let empty_grid = TetrisGrid::default();
    /// # o_tetromino.try_apply(TetrominoMove::HardDrop, &empty_grid);
    /// #
    /// assert_ne!(o_tetromino, Tetromino::new(TetrominoKind::O));
    /// o_tetromino.reset();
    /// assert_eq!(o_tetromino, Tetromino::new(TetrominoKind::O));
    /// ```
    pub fn reset(&mut self) {
        *self = Self::from(self.kind)
    }

    /// Returns `self`'s blocks.
    pub fn blocks(&self) -> &[Position] {
        &self.blocks
    }

    /// Returns `self`'s associated color.
    /// # Examples
    /// ```
    /// # use tetris_core::{Tetromino, TetrominoKind, TetrisColor};
    /// let o_tetromino = Tetromino::new(TetrominoKind::O);
    /// assert_eq!(o_tetromino.color(), TetrisColor::Yellow);
    /// ```
    pub fn color(&self) -> TetrisColor {
        self.kind.into()
    }

    #[cfg(test)]
    pub(super) fn kind(&self) -> TetrominoKind {
        self.kind
    }
}

/// [Tetromino] state machine :
#[doc = simple_mermaid::mermaid!("tetromino/tetromino_state.mmd")]
impl Tetromino {
    /// Translates `self` to its starting position in [TetrisGrid] `grid` if the blocks are free, otherwise returns [GameOverError::BlockOut].
    ///
    /// Note that:
    /// - `self` is assumed to be in its default position and orientation;
    /// - the translation applied depends on the size of `grid`.
    ///
    /// # Examples
    /// ```
    /// # use tetris_core::{Tetromino, TetrominoMove, TetrisGrid, GameOverError};
    /// # let mut tetromino = Tetromino::default();
    /// # let mut full_grid = TetrisGrid::default();
    /// # assert!(tetromino.try_enter_grid(&full_grid).is_ok());
    /// # assert!(tetromino.try_apply(TetrominoMove::Fall, &mut full_grid));
    /// # assert!(tetromino.lock_down(&mut full_grid).is_ok());
    /// # let mut empty_grid = TetrisGrid::default();
    /// #
    /// assert!(Tetromino::default().try_enter_grid(&empty_grid).is_ok());
    ///
    /// assert_eq!(Tetromino::default().try_enter_grid(&full_grid), Err(GameOverError::BlockOut));
    /// ```
    pub fn try_enter_grid(&mut self, grid: &TetrisGrid) -> Result<(), GameOverError> {
        let offset = grid.can_blocks_spawn_on(&self.blocks)?;

        self.translate(&RotationTranslation::translation(offset));
        Ok(())
    }

    /// Applies [TetrominoMove] `tetromino_move` to `self` if the target blocks are free and inside the grid, otherwise returns `false`.
    ///
    /// # Examples
    /// ```
    /// # use tetris_core::{Tetromino, TetrisGrid, TetrominoMove};
    /// # let mut empty_grid = TetrisGrid::default();
    /// # let mut o_tetromino = Tetromino::default();
    /// # assert!(o_tetromino.try_enter_grid(&empty_grid).is_ok());
    /// #
    /// assert!(o_tetromino.try_apply(TetrominoMove::HardDrop, &empty_grid));
    /// assert!(!o_tetromino.try_apply(TetrominoMove::Fall, &empty_grid));
    /// ```
    pub fn try_apply(&mut self, tetromino_move: TetrominoMove, grid: &TetrisGrid) -> bool {
        if tetromino_move.get_rotation_type() == RotationType::Identity {
            self.apply_translation(tetromino_move, grid)
        } else {
            self.apply_rotation(tetromino_move, grid)
        }
    }

    /// Adds `self`'s blocks to [TetrisGrid] `grid`. If these blocks are all above the **Skyline**, returns [GameOverError::LockOut],
    /// otherwise returns the number of cleared lines.
    ///
    /// Note that `self` is assumed to be on free blocks of `grid`, ie [try_enter_grid](Tetromino::try_enter_grid) was called successfully
    /// and since then, only [try_apply](Tetromino::try_apply) was called on `self`.
    ///
    /// The state machine of the [Tetromino] is drawn below.
    ///
    /// # Examples
    /// ```
    /// # use tetris_core::{Tetromino, TetrisGrid, GameOverError, TetrominoMove};
    /// # let mut empty_grid = TetrisGrid::default();
    /// # let mut o_tetromino = Tetromino::default();
    /// #
    /// o_tetromino.reset();
    /// assert!(o_tetromino.try_enter_grid(&empty_grid).is_ok());
    /// assert_eq!(o_tetromino.lock_down(&mut empty_grid), Err(GameOverError::LockOut));
    ///
    /// o_tetromino.try_apply(TetrominoMove::Fall, &empty_grid);
    /// assert!(o_tetromino.lock_down(&mut empty_grid).is_ok());
    /// ```
    pub fn lock_down(self, grid: &mut TetrisGrid) -> Result<u64, GameOverError> {
        grid.add_blocks_and_clear_lines(&self.blocks, self.color())
    }

    fn apply_translation(&mut self, tetromino_move: TetrominoMove, grid: &TetrisGrid) -> bool {
        let translation = RotationTranslation::translation(tetromino_move.get_translation());
        let moved = self.try_move(grid, &translation);

        if tetromino_move.is_repeated() {
            while self.try_move(grid, &translation) {}
        }

        moved
    }

    fn apply_rotation(&mut self, tetromino_move: TetrominoMove, grid: &TetrisGrid) -> bool {
        let rotation_type = tetromino_move.get_rotation_type();

        let wall_kicks_translations =
            TetrominoKind::wall_kick_translations(&self.kind, rotation_type, self.direction);
        for wall_kick in wall_kicks_translations {
            let movement = RotationTranslation::new(wall_kick, rotation_type, &self.center);
            if self.try_move(grid, &movement) {
                return true;
            }
        }

        false
    }

    /// Return whether the tetromino could be moved.
    fn try_move(&mut self, grid: &TetrisGrid, movement: &RotationTranslation) -> bool {
        let mut new_blocks = self.blocks;
        for new_block in new_blocks.iter_mut() {
            if self.kind.is_rotation_center_on_block_center() {
                new_block.move_by(movement);
            } else {
                new_block.move_by_with_offset(movement);
            }
            // Check if `copy` is inside `grid` and on an empty slot.
            if !grid.is_block_available(new_block) {
                return false;
            }
        }
        self.blocks = new_blocks;
        self.direction.move_by(movement);
        self.center.translate_by(movement);
        true
    }

    /// Translate the tetromino.
    fn translate(&mut self, movement: &RotationTranslation) {
        for block in self.blocks.iter_mut() {
            block.translate_by(movement);
        }
        self.center.translate_by(movement);
    }
}

impl Default for Tetromino {
    fn default() -> Self {
        Tetromino::from(TetrominoKind::O)
    }
}

impl Display for Tetromino {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer: Vec<char> =
            concat!("         \n", "         \n", "         \n", "         \n",)
                .chars()
                .collect();

        for block in self.blocks {
            buffer[10 * block.y() as usize + block.x() as usize] = '#';
        }
        buffer[10 * self.center.y() as usize + self.center.x() as usize] = 'C';

        let buffer = String::from_iter(buffer);

        write!(f, "{buffer}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::o_north(TetrominoKind::O, Direction::North, concat!(
            " C#      \n",
            " ##      \n",
            "         \n",
            "         \n",
        ))]
    #[case::o_east(TetrominoKind::O, Direction::East, concat!(
            " C#      \n",
            " ##      \n",
            "         \n",
            "         \n",
        ))]
    #[case::o_south(TetrominoKind::O, Direction::South, concat!(
            " C#      \n",
            " ##      \n",
            "         \n",
            "         \n",
        ))]
    #[case::o_west(TetrominoKind::O, Direction::West, concat!(
            " C#      \n",
            " ##      \n",
            "         \n",
            "         \n",
        ))]
    #[case::i_north(TetrominoKind::I, Direction::North, concat!(
            "         \n",
            "#C##     \n",
            "         \n",
            "         \n",
        ))]
    #[case::i_east(TetrominoKind::I, Direction::East, concat!(
            "  #      \n",
            " C#      \n",
            "  #      \n",
            "  #      \n",
        ))]
    #[case::i_south(TetrominoKind::I, Direction::South, concat!(
            "         \n",
            " C       \n",
            "####     \n",
            "         \n",
        ))]
    #[case::i_west(TetrominoKind::I, Direction::West, concat!(
            " #       \n",
            " C       \n",
            " #       \n",
            " #       \n",
        ))]
    #[case::l_north(TetrominoKind::L, Direction::North, concat!(
            "  #      \n",
            "#C#      \n",
            "         \n",
            "         \n",
        ))]
    #[case::l_east(TetrominoKind::L, Direction::East, concat!(
            " #       \n",
            " C       \n",
            " ##      \n",
            "         \n",
        ))]
    #[case::l_south(TetrominoKind::L, Direction::South, concat!(
            "         \n",
            "#C#      \n",
            "#        \n",
            "         \n",
        ))]
    #[case::l_west(TetrominoKind::L, Direction::West, concat!(
            "##       \n",
            " C       \n",
            " #       \n",
            "         \n",
        ))]
    #[case::j_north(TetrominoKind::J, Direction::North, concat!(
            "#        \n",
            "#C#      \n",
            "         \n",
            "         \n",
        ))]
    #[case::j_east(TetrominoKind::J, Direction::East, concat!(
            " ##      \n",
            " C       \n",
            " #       \n",
            "         \n",
        ))]
    #[case::j_south(TetrominoKind::J, Direction::South, concat!(
            "         \n",
            "#C#      \n",
            "  #      \n",
            "         \n",
        ))]
    #[case::j_west(TetrominoKind::J, Direction::West, concat!(
            " #       \n",
            " C       \n",
            "##       \n",
            "         \n",
        ))]
    #[case::s_north(TetrominoKind::S, Direction::North, concat!(
            " ##      \n",
            "#C       \n",
            "         \n",
            "         \n",
        ))]
    #[case::s_east(TetrominoKind::S, Direction::East, concat!(
            " #       \n",
            " C#      \n",
            "  #      \n",
            "         \n",
        ))]
    #[case::s_south(TetrominoKind::S, Direction::South, concat!(
            "         \n",
            " C#      \n",
            "##       \n",
            "         \n",
        ))]
    #[case::s_west(TetrominoKind::S, Direction::West, concat!(
            "#        \n",
            "#C       \n",
            " #       \n",
            "         \n",
        ))]
    #[case::z_north(TetrominoKind::Z, Direction::North, concat!(
            "##       \n",
            " C#      \n",
            "         \n",
            "         \n",
        ))]
    #[case::z_east(TetrominoKind::Z, Direction::East, concat!(
            "  #      \n",
            " C#      \n",
            " #       \n",
            "         \n",
        ))]
    #[case::z_south(TetrominoKind::Z, Direction::South, concat!(
            "         \n",
            "#C       \n",
            " ##      \n",
            "         \n",
        ))]
    #[case::z_west(TetrominoKind::Z, Direction::West, concat!(
            " #       \n",
            "#C       \n",
            "#        \n",
            "         \n",
        ))]
    fn tetromino_rotation_is_correct(
        #[case] kind: TetrominoKind,
        #[case] direction: Direction,
        #[case] expected_blocks: &str,
    ) {
        let mut tetromino = Tetromino::new(kind);
        let empty_grid = TetrisGrid::default();

        match direction {
            Direction::North => {}
            Direction::East => {
                tetromino.try_apply(TetrominoMove::Clockwise, &empty_grid);
            }
            Direction::South => {
                tetromino.try_apply(TetrominoMove::HalfTurn, &empty_grid);
            }
            Direction::West => {
                tetromino.try_apply(TetrominoMove::Counterclockwise, &empty_grid);
            }
        }

        assert_eq!(tetromino.to_string(), expected_blocks, "{}", tetromino);
        assert_eq!(tetromino.direction, direction);
    }
}
