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
use super::{GameOverError, LineClear, TetrisColor, TetrisGrid, TetrisResult};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tetromino_move::TetrominoMove;

pub(super) use rotation_translation::RotationType;
pub use spatial_primitives::Position;
pub use tetromino_kind::TetrominoKind;

/// Tetromino.
///
/// According to the **Tetris Guideline**, a tetromino is a "\[shape\] made from four connected squares":
///
/// > "There are seven differently shaped tetriminos that fall into a rectangular **Matrix**.
/// > As tetriminos fall, a player may rotate, move, or drop them into their final resting place.
/// > If a row or more of cells is completely filled with **Blocks**, then the line or lines are cleared from the **Matrix**"
///
/// [Tetromino] state machine :
#[doc = simple_mermaid::mermaid!("tetromino/tetromino_state.mmd")]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Tetromino {
    kind: TetrominoKind,
    center: Position,
    pub(super) blocks: [Position; 4],
    direction: Direction,
    is_last_move_rotation: bool,
}

// #[doc = simple_mermaid::mermaid!("tetromino/tetromino_internals.mmd")]
/// Getters.
impl Tetromino {
    /// Returns `self`'s blocks.
    pub fn blocks(&self) -> &[Position] {
        &self.blocks
    }

    /// Returns `self`'s associated [TetrisColor].
    pub fn color(&self) -> TetrisColor {
        self.kind.into()
    }

    /// Returns `self`'s [TetrominoKind].
    pub fn kind(&self) -> TetrominoKind {
        self.kind
    }
}

impl Tetromino {
    /// Creates a new [Tetromino] of [TetrominoKind] `kind` in the default state.
    pub(super) fn new(kind: TetrominoKind) -> Self {
        let positions = kind.get_initial_position();
        Tetromino {
            kind,
            center: positions[0],
            blocks: [positions[1], positions[2], positions[3], positions[4]],
            direction: Direction::default(),
            is_last_move_rotation: false,
        }
    }

    /// Resets `self` in the default state.
    pub(crate) fn reset(&mut self) {
        *self = Self::new(self.kind)
    }

    /// Translates `self` to its starting position in [TetrisGrid] `grid`.
    ///
    /// Note that:
    /// - `self` is assumed to be reset;
    /// - the translation applied depends on the size of `grid`.
    pub(crate) fn enter_grid(&mut self, grid: &TetrisGrid) {
        let translation = RotationTranslation::translation(grid.get_starting_position());
        self.translate(&translation);
    }

    /// Returns [GameOverError::BlockOut] if `self` is not valid in `grid`.
    pub(crate) fn is_valid_in_grid(&self, grid: &TetrisGrid) -> TetrisResult {
        if self
            .blocks
            .iter()
            .all(|block| grid.is_block_available(block))
        {
            Ok(())
        } else {
            Err(GameOverError::BlockOut)
        }
    }

    fn is_in_t_slot(&self, grid: &TetrisGrid) -> bool {
        if self.kind != TetrominoKind::T {
            return false;
        }
        let corners = [
            self.center + Position::new(-1, -1),
            self.center + Position::new(1, -1),
            self.center + Position::new(-1, 1),
            self.center + Position::new(1, 1),
        ];
        let mut nb_available_corners = 0;
        for corner in corners {
            if grid.is_block_available(&corner) {
                nb_available_corners += 1;
            }
        }
        nb_available_corners < 2
    }

    /// Applies [TetrominoMove] `tetromino_move` to `self` if the target blocks are free and inside the grid.
    /// Returns the number of times `tetromino_move` was applied (will be 0 or 1 if the move is not repeated).
    pub(crate) fn try_apply(&mut self, tetromino_move: TetrominoMove, grid: &TetrisGrid) -> u32 {
        if tetromino_move.get_rotation_type() == RotationType::Identity {
            self.apply_translation(tetromino_move, grid)
        } else {
            self.apply_rotation(tetromino_move, grid).into()
        }
    }

    /// Adds `self`'s blocks to [TetrisGrid] `grid`. If these blocks are all above the **Skyline**, returns [GameOverError::LockOut],
    /// otherwise returns the number of cleared lines.
    ///
    /// Note that `self` is assumed to be on free blocks of `grid`, ie [is_valid_in_grid](Tetromino::is_valid_in_grid) was called successfully
    /// and since then, only `self` was only mutated by [try_apply](Tetromino::try_apply) (see state machine schematic).
    pub(crate) fn lock_down(self, grid: &mut TetrisGrid) -> Result<LineClear, GameOverError> {
        let f = if self.is_last_move_rotation && self.is_in_t_slot(grid) {
            LineClear::t_spin
        } else {
            LineClear::new
        };
        grid.add_blocks_and_clear_lines(self.blocks, self.color())
            .map(f)
    }

    fn apply_translation(&mut self, tetromino_move: TetrominoMove, grid: &TetrisGrid) -> u32 {
        let translation = RotationTranslation::translation(tetromino_move.get_translation());
        let mut nb_moves = self.try_move(grid, &translation).into();

        if tetromino_move.is_repeated() {
            while self.try_move(grid, &translation) {
                nb_moves += 1;
            }
        }

        nb_moves
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
        self.is_last_move_rotation = movement.rotation_type != RotationType::Identity;
        true
    }

    fn translate(&mut self, movement: &RotationTranslation) {
        for block in &mut self.blocks {
            block.translate_by(movement);
        }
        self.center.translate_by(movement);
    }
}

impl Default for Tetromino {
    fn default() -> Self {
        Tetromino::new(TetrominoKind::O)
    }
}

impl Display for Tetromino {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer: Vec<char> = concat!(
            "          \n",
            "          \n",
            "          \n",
            "          \n",
        )
        .chars()
        .collect();

        for block in self.blocks {
            buffer[11 * block.y() as usize + block.x() as usize] = '#';
        }
        buffer[11 * self.center.y() as usize + self.center.x() as usize] = 'C';

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
            " C#       \n",
            " ##       \n",
            "          \n",
            "          \n",
        ))]
    #[case::o_east(TetrominoKind::O, Direction::East, concat!(
            " C#       \n",
            " ##       \n",
            "          \n",
            "          \n",
        ))]
    #[case::o_south(TetrominoKind::O, Direction::South, concat!(
            " C#       \n",
            " ##       \n",
            "          \n",
            "          \n",
        ))]
    #[case::o_west(TetrominoKind::O, Direction::West, concat!(
            " C#       \n",
            " ##       \n",
            "          \n",
            "          \n",
        ))]
    #[case::i_north(TetrominoKind::I, Direction::North, concat!(
            "          \n",
            "#C##      \n",
            "          \n",
            "          \n",
        ))]
    #[case::i_east(TetrominoKind::I, Direction::East, concat!(
            "  #       \n",
            " C#       \n",
            "  #       \n",
            "  #       \n",
        ))]
    #[case::i_south(TetrominoKind::I, Direction::South, concat!(
            "          \n",
            " C        \n",
            "####      \n",
            "          \n",
        ))]
    #[case::i_west(TetrominoKind::I, Direction::West, concat!(
            " #        \n",
            " C        \n",
            " #        \n",
            " #        \n",
        ))]
    #[case::l_north(TetrominoKind::L, Direction::North, concat!(
            "  #       \n",
            "#C#       \n",
            "          \n",
            "          \n",
        ))]
    #[case::l_east(TetrominoKind::L, Direction::East, concat!(
            " #        \n",
            " C        \n",
            " ##       \n",
            "          \n",
        ))]
    #[case::l_south(TetrominoKind::L, Direction::South, concat!(
            "          \n",
            "#C#       \n",
            "#         \n",
            "          \n",
        ))]
    #[case::l_west(TetrominoKind::L, Direction::West, concat!(
            "##        \n",
            " C        \n",
            " #        \n",
            "          \n",
        ))]
    #[case::j_north(TetrominoKind::J, Direction::North, concat!(
            "#         \n",
            "#C#       \n",
            "          \n",
            "          \n",
        ))]
    #[case::j_east(TetrominoKind::J, Direction::East, concat!(
            " ##       \n",
            " C        \n",
            " #        \n",
            "          \n",
        ))]
    #[case::j_south(TetrominoKind::J, Direction::South, concat!(
            "          \n",
            "#C#       \n",
            "  #       \n",
            "          \n",
        ))]
    #[case::j_west(TetrominoKind::J, Direction::West, concat!(
            " #        \n",
            " C        \n",
            "##        \n",
            "          \n",
        ))]
    #[case::s_north(TetrominoKind::S, Direction::North, concat!(
            " ##       \n",
            "#C        \n",
            "          \n",
            "          \n",
        ))]
    #[case::s_east(TetrominoKind::S, Direction::East, concat!(
            " #        \n",
            " C#       \n",
            "  #       \n",
            "          \n",
        ))]
    #[case::s_south(TetrominoKind::S, Direction::South, concat!(
            "          \n",
            " C#       \n",
            "##        \n",
            "          \n",
        ))]
    #[case::s_west(TetrominoKind::S, Direction::West, concat!(
            "#         \n",
            "#C        \n",
            " #        \n",
            "          \n",
        ))]
    #[case::z_north(TetrominoKind::Z, Direction::North, concat!(
            "##        \n",
            " C#       \n",
            "          \n",
            "          \n",
        ))]
    #[case::z_east(TetrominoKind::Z, Direction::East, concat!(
            "  #       \n",
            " C#       \n",
            " #        \n",
            "          \n",
        ))]
    #[case::z_south(TetrominoKind::Z, Direction::South, concat!(
            "          \n",
            "#C        \n",
            " ##       \n",
            "          \n",
        ))]
    #[case::z_west(TetrominoKind::Z, Direction::West, concat!(
            " #        \n",
            "#C        \n",
            "#         \n",
            "          \n",
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
                tetromino.try_apply(RotationType::Clockwise.into(), &empty_grid);
            }
            Direction::South => {
                tetromino.try_apply(RotationType::HalfTurn.into(), &empty_grid);
            }
            Direction::West => {
                tetromino.try_apply(RotationType::Counterclockwise.into(), &empty_grid);
            }
        }

        assert_eq!(tetromino.to_string(), expected_blocks, "{}", tetromino);
        assert_eq!(tetromino.direction, direction);
    }

    #[rstest]
    fn reset_is_correct(#[values(TetrominoKind::O, TetrominoKind::I)] kind: TetrominoKind) {
        let mut tetromino = Tetromino::new(kind);
        let empty_grid = TetrisGrid::default();
        tetromino.try_apply(RotationType::Clockwise.into(), &empty_grid);
        tetromino.try_apply(TetrominoMove::repeat(Position::FALL), &empty_grid);

        assert_ne!(tetromino, Tetromino::new(kind));
        assert_ne!(tetromino.direction, Direction::North);

        tetromino.reset();
        assert_eq!(tetromino, Tetromino::new(kind));
        assert_eq!(tetromino.direction, Direction::North);
    }

    // should check that the grid size is taken into account
    // should check that occupancy is taken into account
    // easier in an integration test
    #[rstest]
    fn enter_grid_and_check_is_correct() {
        let mut tetromino = Tetromino::new(TetrominoKind::O);

        let mut full_grid = TetrisGrid::default();
        tetromino.enter_grid(&full_grid);
        assert!(tetromino.is_valid_in_grid(&full_grid).is_ok());
        assert_eq!(
            tetromino.try_apply(TetrominoMove::translation(Position::FALL), &full_grid),
            1
        );
        assert!(tetromino.lock_down(&mut full_grid).is_ok());

        let empty_grid = TetrisGrid::default();
        let mut tetromino = Tetromino::default();
        tetromino.enter_grid(&empty_grid);
        assert!(tetromino.is_valid_in_grid(&empty_grid).is_ok());

        let mut tetromino = Tetromino::default();
        tetromino.enter_grid(&full_grid);
        assert_eq!(
            tetromino.is_valid_in_grid(&full_grid),
            Err(GameOverError::BlockOut)
        );
    }

    // # Examples
    // ```
    // # use core_tetris::{Tetromino, TetrisGrid, GameOverError, TetrominoMove};
    // # let mut empty_grid = TetrisGrid::default();
    // #
    // let mut o_tetromino = Tetromino::default();
    // assert!(o_tetromino.try_enter_grid(&empty_grid).is_ok());
    // assert_eq!(o_tetromino.lock_down(&mut empty_grid), Err(GameOverError::LockOut));
    //
    // let mut o_tetromino = Tetromino::default();
    // assert!(o_tetromino.try_enter_grid(&empty_grid).is_ok());
    // assert!(o_tetromino.try_apply(TetrominoMove::Fall, &empty_grid));
    // assert!(o_tetromino.lock_down(&mut empty_grid).is_ok());
    // ```
    //

    //
    // # Examples
    // ```
    // # use core_tetris::{Tetromino, TetrisGrid, TetrominoMove};
    // # let mut empty_grid = TetrisGrid::default();
    // # let mut o_tetromino = Tetromino::default();
    // # assert!(o_tetromino.try_enter_grid(&empty_grid).is_ok());
    // #
    // assert!(o_tetromino.try_apply(TetrominoMove::HardDrop, &empty_grid));
    // assert!(!o_tetromino.try_apply(TetrominoMove::Fall, &empty_grid));
    // ```
}
