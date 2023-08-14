//! Defines a Tetromino piece and functions used by all 7 types of Tetromino.
use super::block::{Block, Collision};
use super::point::{Point, Transform};
use super::rotation_state::{RotationState, RotationStateImplementation};
use super::tetris_grid::GridLine;
use super::tetromino_kind::{TetrominoKind, TetrominoKindImplementation};
use super::translation_rotation::{RotationType, TranslationRotation};
use core::fmt::Display;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Tetromino piece among the 7 kinds in the game positioned on the grid.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Tetromino {
    kind: TetrominoKind,
    center: Point,
    pub blocks: [Block; 4],
    rotation_status: RotationState,
    pub is_ghost: bool,
}

/// Uses of a Tetromino by a Player : includes creating and moving pieces.
pub trait PlayerTetromino {
    /// Moves the Tetromino down one cell if it's possible.
    fn fall(&mut self, matrix: &[GridLine]) -> Result<(), ()>;

    /// Moves the Tetromino as far down as possible.
    fn hard_drop(&mut self, matrix: &[GridLine]);

    /// Moves the Tetromino one cell to the left if it's possible.
    fn left(&mut self, matrix: &[GridLine]);

    /// Moves the Tetromino one cell to the right if it's possible.
    fn right(&mut self, matrix: &[GridLine]);

    /// Turns the Tetromino clockwise if it's possible, eventually using wall-kicks.
    fn turn_clockwise(&mut self, matrix: &[GridLine]);

    /// Turns the Tetromino counterclockwise if it's possible, eventually using wall-kicks.
    fn turn_counterclockwise(&mut self, matrix: &[GridLine]);

    /// Returns the resulting position of the Tetromino Blocks if the movement is possible.
    fn check_possible(
        &self,
        matrix: &[GridLine],
        movement: TranslationRotation,
    ) -> Result<[Block; 4], ()>;

    /// Returns an Option eventually containing a Tetromino if its starting position is empty.
    fn new(kind: TetrominoKind, matrix: &[GridLine]) -> Option<Tetromino>;

    /// Returns a Tetromino at its starting position without checking that this place is empty.
    fn new_unchecked(kind: TetrominoKind) -> Tetromino;

    /// Resets the Tetromino at its starting position.
    fn reset_position(&mut self);

    /// Returns a ghost copy of the Tetromino.
    fn make_ghost_copy(&mut self) -> Tetromino;
}

impl PlayerTetromino for Tetromino {
    fn fall(&mut self, matrix: &[Vec<Option<Block>>]) -> Result<(), ()> {
        self.blocks = self.check_possible(matrix, TranslationRotation::fall())?;
        self.center.go_down();
        Ok(())
    }

    fn hard_drop(&mut self, matrix: &[GridLine]) {
        match self.fall(matrix) {
            Err(()) => {}
            Ok(()) => self.hard_drop(matrix),
        }
    }

    fn left(&mut self, matrix: &[GridLine]) {
        if let Ok(new_blocks) = self.check_possible(matrix, TranslationRotation::left()) {
            self.blocks = new_blocks;
            self.center.go_left();
        }
    }

    fn right(&mut self, matrix: &[GridLine]) {
        if let Ok(new_blocks) = self.check_possible(matrix, TranslationRotation::right()) {
            self.blocks = new_blocks;
            self.center.go_right();
        }
    }

    fn turn_clockwise(&mut self, matrix: &[GridLine]) {
        if self.kind == TetrominoKind::O {
            return;
        };
        let wall_kicks_translations = TetrominoKind::wall_kicks_translations(
            &self.kind,
            RotationType::Clockwise,
            self.rotation_status,
        );
        for wall_kick in &wall_kicks_translations {
            match self.check_possible(
                matrix,
                TranslationRotation::new(*wall_kick, RotationType::Clockwise, &self.center),
            ) {
                Err(()) => {
                    continue;
                }
                Ok(new_blocks) => {
                    self.blocks = new_blocks;
                    self.rotation_status.clockwise();
                    self.center += *wall_kick;
                    return;
                }
            }
        }
    }

    fn turn_counterclockwise(&mut self, matrix: &[GridLine]) {
        if self.kind == TetrominoKind::O {
            return;
        };
        let wall_kicks_translations = TetrominoKind::wall_kicks_translations(
            &self.kind,
            RotationType::Counterclockwise,
            self.rotation_status,
        );
        for wall_kick in &wall_kicks_translations {
            match self.check_possible(
                matrix,
                TranslationRotation::new(*wall_kick, RotationType::Counterclockwise, &self.center),
            ) {
                Err(()) => {
                    continue;
                }
                Ok(new_blocks) => {
                    self.blocks = new_blocks;
                    self.rotation_status.counterclockwise();
                    self.center += *wall_kick;
                    return;
                }
            }
        }
    }

    fn check_possible(
        &self,
        matrix: &[GridLine],
        movement: TranslationRotation,
    ) -> Result<[Block; 4], ()> {
        let mut new_blocks = vec![];
        for i in 0..4 {
            new_blocks.push(self.blocks[i].move_to(matrix, &movement)?);
        }
        let blocks = [new_blocks[0], new_blocks[1], new_blocks[2], new_blocks[3]];
        Ok(blocks)
    }

    fn new(kind: TetrominoKind, matrix: &[GridLine]) -> Option<Tetromino> {
        let positions = kind.get_initial_position();
        let color = kind.get_color();
        for i in 1..5 {
            if matrix[positions[2 * i + 1] as usize][positions[2 * i] as usize].is_some() {
                return None;
            }
        }
        Some(Tetromino {
            kind,
            center: Point::new(positions[0], positions[1]),
            blocks: [
                Block::new(color, positions[2], positions[3]),
                Block::new(color, positions[4], positions[5]),
                Block::new(color, positions[6], positions[7]),
                Block::new(color, positions[8], positions[9]),
            ],
            rotation_status: RotationState::R0,
            is_ghost: false,
        })
    }

    fn new_unchecked(kind: TetrominoKind) -> Tetromino {
        let positions = kind.get_initial_position();
        let color = kind.get_color();
        Tetromino {
            kind,
            center: Point::new(positions[0], positions[1]),
            blocks: [
                Block::new(color, positions[2], positions[3]),
                Block::new(color, positions[4], positions[5]),
                Block::new(color, positions[6], positions[7]),
                Block::new(color, positions[8], positions[9]),
            ],
            rotation_status: RotationState::R0,
            is_ghost: false,
        }
    }

    fn reset_position(&mut self) {
        let positions = self.kind.get_initial_position();
        let color = self.kind.get_color();
        self.center = Point::new(positions[0], positions[1]);
        self.blocks = [
            Block::new(color, positions[2], positions[3]),
            Block::new(color, positions[4], positions[5]),
            Block::new(color, positions[6], positions[7]),
            Block::new(color, positions[8], positions[9]),
        ];
    }

    fn make_ghost_copy(&mut self) -> Tetromino {
        let mut ghost = *self;
        ghost.is_ghost = true;
        ghost
    }
}

impl Default for Tetromino {
    fn default() -> Self {
        Tetromino {
            kind: TetrominoKind::O,
            center: Point::default(),
            blocks: [Block::default(); 4],
            rotation_status: RotationState::R0,
            is_ghost: false,
        }
    }
}

impl Display for Tetromino {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.kind.get())
    }
}

impl Tetromino {
    /// Returns the 4 Blocks of the Tetromino.
    pub(in crate::tetris_back_end) fn split(&mut self) -> [Block; 4] {
        self.blocks
    }
}
