//! Defines the implementation of [Tetromino](super::Tetromino).
use super::{
    block::{Block, Collision},
    point::{Point, Transform},
    rotation_state::{RotationState, RotationStateUpdate},
    translation_rotation::RotationType,
    GridLine, GridMatrix, Tetromino, TetrominoKind, TranslationRotation,
};
use core::fmt::Display;
use std::fmt::Formatter;

impl Tetromino {
    /// Moves the Tetromino down one cell if it's possible.
    pub fn fall(&mut self, matrix: &[GridLine]) -> Result<(), ()> {
        self.blocks = self.check_possible(matrix, TranslationRotation::fall())?;
        self.center.go_down();
        Ok(())
    }

    /// Moves the Tetromino as far down as possible.
    pub fn hard_drop(&mut self, matrix: &GridMatrix) {
        match self.fall(matrix) {
            Err(()) => {}
            Ok(()) => self.hard_drop(matrix),
        }
    }

    /// Moves the Tetromino one cell to the left if it's possible.
    pub fn left(&mut self, matrix: &GridMatrix) {
        if let Ok(new_blocks) = self.check_possible(matrix, TranslationRotation::left()) {
            self.blocks = new_blocks;
            self.center.go_left();
        }
    }

    /// Moves the Tetromino one cell to the right if it's possible.
    pub fn right(&mut self, matrix: &GridMatrix) {
        if let Ok(new_blocks) = self.check_possible(matrix, TranslationRotation::right()) {
            self.blocks = new_blocks;
            self.center.go_right();
        }
    }

    /// Turns the Tetromino clockwise if it's possible, eventually using wall-kicks.
    pub fn turn_clockwise(&mut self, matrix: &GridMatrix) {
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

    /// Turns the Tetromino counterclockwise if it's possible, eventually using wall-kicks.
    pub fn turn_counterclockwise(&mut self, matrix: &GridMatrix) {
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

    /// Returns the resulting position of the Tetromino Blocks if the movement is possible.
    pub fn check_possible(
        &self,
        matrix: &GridMatrix,
        movement: TranslationRotation,
    ) -> Result<[Block; 4], ()> {
        let mut new_blocks = vec![];
        for i in 0..4 {
            new_blocks.push(self.blocks[i].move_to(matrix, &movement)?);
        }
        let blocks = [new_blocks[0], new_blocks[1], new_blocks[2], new_blocks[3]];
        Ok(blocks)
    }

    /// Returns an Option eventually containing a Tetromino if its starting position is empty.
    pub fn new(kind: TetrominoKind, matrix: &GridMatrix) -> Option<Tetromino> {
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

    /// Returns a Tetromino at its starting position without checking that this place is empty.
    pub fn new_unchecked(kind: TetrominoKind) -> Tetromino {
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

    /// Resets the Tetromino at its starting position.
    pub fn reset_position(&mut self) {
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
    pub(super) fn split(&mut self) -> [Block; 4] {
        self.blocks
    }
}
