//! Define the implementation of [Tetromino].
#[cfg(test)]
use super::spatial_primitives::{FALL, LEFT, RIGHT, RISE};
use super::{
    moving_primitives::BackEndError, ApplyRotationTranslation, Block, Direction, Position,
    RotationTranslation, RotationType, TetrisGrid, Tetromino, TetrominoKind, UseTetromino,
};
use core::fmt::Display;
use std::fmt::Formatter;

impl UseTetromino for Tetromino {
    fn fall(&mut self, grid: &TetrisGrid) -> Result<(), ()> {
        let movement = RotationTranslation::fall();
        self.try_move(grid, &movement).map_err(|_err| ())
    }

    fn hard_drop(&mut self, grid: &TetrisGrid) {
        if self.fall(grid).is_ok() {
            self.hard_drop(grid);
        }
    }

    fn left(&mut self, grid: &TetrisGrid) {
        let movement = RotationTranslation::left();
        let _ = self.try_move(grid, &movement);
    }

    fn right(&mut self, grid: &TetrisGrid) {
        let movement = RotationTranslation::right();
        let _ = self.try_move(grid, &movement);
    }

    fn turn_clockwise(&mut self, grid: &TetrisGrid) {
        if self.kind == TetrominoKind::O {
            return;
        };
        let wall_kicks_translations = TetrominoKind::wall_kick_translations(
            &self.kind,
            RotationType::Clockwise,
            self.direction,
        );
        for wall_kick in wall_kicks_translations {
            let movement =
                RotationTranslation::new(wall_kick, RotationType::Clockwise, &self.center);
            if self.try_move(grid, &movement).is_ok() {
                return;
            }
        }
    }

    fn turn_counterclockwise(&mut self, grid: &TetrisGrid) {
        if self.kind == TetrominoKind::O {
            return;
        };
        let wall_kicks_translations = TetrominoKind::wall_kick_translations(
            &self.kind,
            RotationType::Counterclockwise,
            self.direction,
        );
        for wall_kick in wall_kicks_translations {
            let movement =
                RotationTranslation::new(wall_kick, RotationType::Counterclockwise, &self.center);
            if self.try_move(grid, &movement).is_ok() {
                return;
            }
        }
    }

    fn new(kind: TetrominoKind, grid: &TetrisGrid) -> Option<Tetromino> {
        let positions = kind.get_initial_position();
        let color = kind.into();
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

    fn new_unchecked(kind: TetrominoKind) -> Tetromino {
        let positions = kind.get_initial_position();
        let color = kind.into();
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

    fn reset_position(&mut self) {
        let positions = self.kind.get_initial_position();
        let color = self.kind.into();
        self.center = positions[0];
        self.blocks = [
            Block::from(color, positions[1]),
            Block::from(color, positions[2]),
            Block::from(color, positions[3]),
            Block::from(color, positions[4]),
        ];
    }

    fn make_ghost_copy(&mut self) -> Tetromino {
        let mut ghost = *self;
        ghost.is_ghost = true;
        ghost
    }
}

impl Tetromino {
    /// Return whether the tetromino could be moved.
    pub(super) fn try_move(
        &mut self,
        grid: &TetrisGrid,
        movement: &RotationTranslation,
    ) -> Result<(), BackEndError> {
        let mut new_blocks = self.blocks;
        for new_block in new_blocks.iter_mut() {
            new_block.try_move(
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
            let _ = naive_i_tetromino.try_move(&empty_grid, &corrected_rotation);

            i_tetromino.turn_clockwise(&empty_grid);

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
            let _ = naive_i_tetromino.try_move(&empty_grid, &corrected_rotation);

            i_tetromino.turn_counterclockwise(&empty_grid);

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
