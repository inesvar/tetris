//! Define `trait` [UseTetromino] for [Tetromino].
#[cfg(test)]
use super::spatial_primitives::{FALL, LEFT, RIGHT, RISE};
use super::{
    moving_primitives::ApplyRotationTranslation,
    rotation_translation::{RotationTranslation, RotationType},
    spatial_primitives::{Direction, Position},
    tetris_block::{Block, TryMoveBlock},
    tetris_grid::BackendError,
    Deserialize, Pcg32, Serialize, TetrisColor, TetrisGrid, TetrominoKind, UseTetromino,
};
use core::fmt::Display;
use rand::seq::SliceRandom;
use std::fmt::Formatter;

/// Tetromino.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub(crate) struct Tetromino {
    kind: TetrominoKind,
    center: Position,
    pub(super) blocks: [Block; 4],
    direction: Direction,
    pub(super) is_ghost: bool,
}

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

    fn turn_half_turn(&mut self, grid: &TetrisGrid) {
        if self.kind == TetrominoKind::O {
            return;
        };
        let movement = RotationTranslation::rotation(RotationType::HalfTurn, &self.center);
        let _ = self.try_move(grid, &movement).is_ok();
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

    fn new_tetromino_bag(requested_size_of_bag: u32, rng: &mut Pcg32) -> Vec<TetrominoKind> {
        let mut size_of_bag = requested_size_of_bag;
        if size_of_bag == 0 {
            size_of_bag = 1;
        }
        let mut tetromino_bag = vec![];
        let mut list = vec![];
        for _ in 0..(size_of_bag / 7) {
            for i in 0..7 {
                list.push(i);
            }
        }
        if size_of_bag % 7 != 0 {
            for i in 0..7 {
                list.push(i);
            }
        }
        // the list now has k elements, where k is the lower multiple of 7 higher or equal to size_of_bag
        list.shuffle(rng);

        for i in 0..size_of_bag {
            // only the first size_of_bag elements are used
            match list[i as usize] {
                0 => tetromino_bag.push(TetrominoKind::I),
                1 => tetromino_bag.push(TetrominoKind::O),
                2 => tetromino_bag.push(TetrominoKind::T),
                3 => tetromino_bag.push(TetrominoKind::S),
                4 => tetromino_bag.push(TetrominoKind::Z),
                5 => tetromino_bag.push(TetrominoKind::J),
                _ => tetromino_bag.push(TetrominoKind::L),
            }
        }
        tetromino_bag
    }
}

impl Tetromino {
    /// Return whether the tetromino could be moved.
    pub(super) fn try_move(
        &mut self,
        grid: &TetrisGrid,
        movement: &RotationTranslation,
    ) -> Result<(), BackendError> {
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

    pub(in crate::app::player) fn is_valid_in(&self, grid: &TetrisGrid) -> bool {
        grid.are_blocks_available(&self.blocks)
    }

    pub(in crate::app::player) fn get_tetris_color(&self) -> TetrisColor {
        self.blocks[0].color()
    }

    pub(in crate::app::player) fn lock_down(
        self,
        grid: &mut TetrisGrid,
    ) -> Result<u64, BackendError> {
        grid.add_blocks(&self.blocks, self.get_tetris_color())
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

#[cfg(test)]
impl Tetromino {
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
    use crate::settings::{NB_COLUMNS, NB_HIDDEN_ROWS, NB_ROWS};

    #[test]
    fn i_tetromino_rotation_is_correct() {
        let empty_grid = TetrisGrid::new(NB_COLUMNS, NB_ROWS, NB_HIDDEN_ROWS);
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
