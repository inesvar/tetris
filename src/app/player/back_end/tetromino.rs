//! Define the implementation of [Tetromino].
#[cfg(test)]
use super::spatial_primitives::{FALL, LEFT, RIGHT, RISE};
use super::{
    ApplyRotationTranslation, Block, Direction, Position, RotationTranslation, RotationType,
    TetrisGrid, Tetromino, TetrominoKind,
};
use core::fmt::Display;
use std::fmt::Formatter;

impl Tetromino {
    /// Return whether the tetromino could be moved one cell down.
    pub(in crate::app::player) fn fall(&mut self, grid: &TetrisGrid) -> Result<(), ()> {
        let movement = RotationTranslation::fall();
        self.move_if_ok(grid, &movement)
    }

    /// Move the tetromino down until it's not possible anymore.
    pub(in crate::app::player) fn hard_drop(&mut self, grid: &TetrisGrid) {
        if self.fall(grid).is_ok() {
            self.hard_drop(grid);
        }
    }

    /// Move the tetromino one cell left if it's possible.
    pub(in crate::app::player) fn left(&mut self, grid: &TetrisGrid) {
        let movement = RotationTranslation::left();
        let _ = self.move_if_ok(grid, &movement);
    }

    /// Move the tetromino one cell right if it's possible.
    pub(in crate::app::player) fn right(&mut self, grid: &TetrisGrid) {
        let movement = RotationTranslation::right();
        let _ = self.move_if_ok(grid, &movement);
    }

    /// Turn the tetromino clockwise if it's possible, eventually using wall-kicks.
    pub(in crate::app::player) fn turn_clockwise(&mut self, grid: &TetrisGrid) {
        if self.kind == TetrominoKind::O {
            return;
        };
        let wall_kicks_translations = TetrominoKind::wall_kick_translations(
            &self.kind,
            RotationType::Clockwise,
            self.direction,
        );
        for wall_kick in &wall_kicks_translations {
            let movement =
                RotationTranslation::new(wall_kick, RotationType::Clockwise, &self.center);
            if self.move_if_ok(grid, &movement).is_ok() {
                return;
            }
        }
    }

    /// Turn the tetromino counterclockwise if it's possible, eventually using wall-kicks.
    pub(in crate::app::player) fn turn_counterclockwise(&mut self, grid: &TetrisGrid) {
        if self.kind == TetrominoKind::O {
            return;
        };
        let wall_kicks_translations = TetrominoKind::wall_kick_translations(
            &self.kind,
            RotationType::Counterclockwise,
            self.direction,
        );
        for wall_kick in &wall_kicks_translations {
            let movement =
                RotationTranslation::new(wall_kick, RotationType::Counterclockwise, &self.center);
            if self.move_if_ok(grid, &movement).is_ok() {
                return;
            }
        }
    }

    /// Return whether the tetromino could be moved.
    pub(super) fn move_if_ok(
        &mut self,
        grid: &TetrisGrid,
        movement: &RotationTranslation,
    ) -> Result<(), ()> {
        let mut new_blocks = self.blocks;
        for (i, new_block) in new_blocks.iter_mut().enumerate() {
            *new_block = self.blocks[i].can_be_moved(
                grid,
                movement,
                self.kind.is_rotation_center_on_block_center(),
            )?;
        }
        self.blocks = new_blocks;
        self.direction.turn_by(movement);
        self.center.translate_by(movement);
        Ok(())
    }

    /// Returns an Option eventually containing a Tetromino if its starting position is empty.
    pub(in crate::app::player) fn new(kind: TetrominoKind, grid: &TetrisGrid) -> Option<Tetromino> {
        let positions = kind.get_initial_position();
        let color = kind.get_color();
        for position in positions {
            if grid.matrix[position.x as usize][position.y as usize].is_some() {
                return None;
            }
        }
        Some(Tetromino {
            kind,
            center: positions[0],
            blocks: [
                Block::from(color, positions[1]),
                Block::from(color, positions[2]),
                Block::from(color, positions[3]),
                Block::from(color, positions[4]),
            ],
            direction: Direction::default(),
            is_ghost: false,
        })
    }

    /// Returns a Tetromino at its starting position without checking that this place is empty.
    pub fn new_unchecked(kind: TetrominoKind) -> Tetromino {
        let positions = kind.get_initial_position();
        let color = kind.get_color();
        Tetromino {
            kind,
            center: positions[0],
            blocks: [
                Block::from(color, positions[1]),
                Block::from(color, positions[2]),
                Block::from(color, positions[3]),
                Block::from(color, positions[4]),
            ],
            direction: Direction::default(),
            is_ghost: false,
        }
    }

    // TODO can't this cause a collision??
    // TODO the direction is not reset ??? => test this but using new_unchecked seems better
    /// Resets the Tetromino at its starting position.
    pub fn reset_position(&mut self) {
        let positions = self.kind.get_initial_position();
        let color = self.kind.get_color();
        self.center = positions[0];
        self.blocks = [
            Block::from(color, positions[1]),
            Block::from(color, positions[2]),
            Block::from(color, positions[3]),
            Block::from(color, positions[4]),
        ];
    }

    /// Returns a ghost copy of the Tetromino.
    pub fn make_ghost_copy(&mut self) -> Tetromino {
        let mut ghost = *self;
        ghost.is_ghost = true;
        ghost
    }
}

impl Default for Tetromino {
    fn default() -> Self {
        Tetromino {
            kind: TetrominoKind::O,
            center: Position::default(),
            blocks: [Block::default(); 4],
            direction: Direction::default(),
            is_ghost: false,
        }
    }
}

// Required by `CircularBuffer`.
impl Display for Tetromino {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.kind)
    }
}

impl Tetromino {
    /// Returns the 4 Blocks of the Tetromino.
    pub(super) fn split(&mut self) -> [Block; 4] {
        self.blocks
    }

    pub(super) fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    #[cfg(test)]
    fn i_tetromino_rotation_correction(&mut self, movement: &RotationTranslation) -> Position {
        match (self.direction, movement.rotation_type) {
            (Direction::Up, RotationType::Clockwise) => RIGHT,
            (Direction::Right, RotationType::Counterclockwise) => LEFT,
            (Direction::Right, RotationType::Clockwise) => FALL,
            (Direction::Down, RotationType::Counterclockwise) => RISE,
            (Direction::Down, RotationType::Clockwise) => LEFT,
            (Direction::Left, RotationType::Counterclockwise) => RIGHT,
            (Direction::Left, RotationType::Clockwise) => RISE,
            (Direction::Up, RotationType::Counterclockwise) => FALL,
            (_, _) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{NB_COLUMNS, NB_ROWS};

    #[test]
    fn i_tetromino_rotation_is_correct() {
        let empty_grid = TetrisGrid::new(NB_COLUMNS, NB_ROWS);
        let mut i_tetromino = Tetromino::new_unchecked(TetrominoKind::I);
        let mut naive_i_tetromino = Tetromino::new_unchecked(TetrominoKind::I);
        // regular rotation will be used for tetromino T
        naive_i_tetromino.kind = TetrominoKind::T;

        println!(
            "{:?}\n{:?}\n{:?}\n{:?}",
            i_tetromino.blocks,
            naive_i_tetromino.blocks,
            i_tetromino.center,
            naive_i_tetromino.center
        );
        assert_eq!(i_tetromino.blocks, naive_i_tetromino.blocks);

        for _ in 0..4 {
            let rotation = RotationTranslation {
                rotation_type: RotationType::Clockwise,
                rotation_center: naive_i_tetromino.center,
                translation: Position::new(0, 0),
            };

            let translation = naive_i_tetromino.i_tetromino_rotation_correction(&rotation);
            let mut corrected_rotation = rotation;
            corrected_rotation.translation = translation;
            let _ = naive_i_tetromino.move_if_ok(&empty_grid, &corrected_rotation);

            let _ = i_tetromino.turn_clockwise(&empty_grid);

            println!(
                "{:?}\n{:?}\n{:?}\n{:?}",
                i_tetromino.blocks,
                naive_i_tetromino.blocks,
                i_tetromino.center,
                naive_i_tetromino.center
            );

            assert_eq!(i_tetromino.blocks, naive_i_tetromino.blocks);
        }

        for _ in 0..4 {
            let rotation = RotationTranslation {
                rotation_type: RotationType::Counterclockwise,
                rotation_center: naive_i_tetromino.center,
                translation: Position::new(0, 0),
            };
            let translation = naive_i_tetromino.i_tetromino_rotation_correction(&rotation);
            let mut corrected_rotation = rotation;
            corrected_rotation.translation = translation;
            let _ = naive_i_tetromino.move_if_ok(&empty_grid, &corrected_rotation);

            let _ = i_tetromino.turn_counterclockwise(&empty_grid);

            println!(
                "{:?}\n{:?}\n{:?}\n{:?}",
                i_tetromino.blocks,
                naive_i_tetromino.blocks,
                i_tetromino.center,
                naive_i_tetromino.center
            );

            assert_eq!(i_tetromino.blocks, naive_i_tetromino.blocks);
        }
    }
}
