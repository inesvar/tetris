//! Define `trait` [UseTetromino] for [Tetromino].
#[cfg(test)]
use super::spatial_primitives::{FALL, LEFT, RIGHT, RISE};
use super::{
    moving_primitives::ApplyRotationTranslation,
    rotation_translation::{RotationTranslation, RotationType},
    spatial_primitives::{Direction, Position},
    tetris_grid::GameOverError,
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
    pub(super) blocks: [Position; 4],
    pub(in crate::app::player) color: TetrisColor,
    direction: Direction,
}

impl UseTetromino for Tetromino {
    fn try_fall(&mut self, grid: &TetrisGrid) -> bool {
        let movement = RotationTranslation::fall();
        self.try_move(grid, &movement)
    }

    fn hard_drop(&mut self, grid: &TetrisGrid) {
        if self.try_fall(grid) {
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
            if self.try_move(grid, &movement) {
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
            if self.try_move(grid, &movement) {
                return;
            }
        }
    }

    fn new(kind: TetrominoKind) -> Tetromino {
        let positions = kind.get_initial_position();
        Tetromino {
            kind,
            center: positions[0],
            blocks: [positions[1], positions[2], positions[3], positions[4]],
            color: kind.into(),
            direction: Direction::default(),
        }
    }

    fn reset(&mut self) {
        let positions = self.kind.get_initial_position();
        self.center = positions[0];
        self.blocks = [positions[1], positions[2], positions[3], positions[4]];
        self.direction = Direction::default();
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

    fn can_enter_grid(&self, grid: &TetrisGrid) -> Result<(), GameOverError> {
        grid.can_blocks_spawn_on(&self.blocks)
    }

    fn lock_down(self, grid: &mut TetrisGrid) -> Result<u64, GameOverError> {
        grid.add_blocks(&self.blocks, self.color)
    }
}

impl Tetromino {
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
        self.direction.turn_around_block_center(movement);
        self.center.translate_by(movement);
        true
    }

    pub(in crate::app::player) fn blocks(&self) -> &[Position] {
        &self.blocks
    }
}

/// `Tetromino::default` is far from the origin and is not renderer on the screen.
impl Default for Tetromino {
    fn default() -> Self {
        Tetromino {
            kind: TetrominoKind::O,
            center: Position::default(),
            blocks: [Position::new(-50, -50); 4],
            color: TetrisColor::Yellow,
            direction: Direction::default(),
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
            (Direction::North, RotationType::Clockwise) => RIGHT,
            (Direction::East, RotationType::Counterclockwise) => LEFT,
            (Direction::East, RotationType::Clockwise) => FALL,
            (Direction::South, RotationType::Counterclockwise) => RISE,
            (Direction::South, RotationType::Clockwise) => LEFT,
            (Direction::West, RotationType::Counterclockwise) => RIGHT,
            (Direction::West, RotationType::Clockwise) => RISE,
            (Direction::North, RotationType::Counterclockwise) => FALL,
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
        let mut i_tetromino = Tetromino::new(TetrominoKind::I);
        let mut naive_i_tetromino = Tetromino::new(TetrominoKind::I);
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
