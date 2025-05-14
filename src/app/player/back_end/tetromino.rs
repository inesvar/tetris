//! Define the implementation of [Tetromino].
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
        let wall_kicks_translations = TetrominoKind::wall_kicks_translations(
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
        let wall_kicks_translations = TetrominoKind::wall_kicks_translations(
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
        let mut new_blocks = [Block::default(); 4];
        for (i, new_block) in new_blocks.iter_mut().enumerate() {
            *new_block = self.blocks[i].can_be_moved(grid, movement)?;
        }
        self.blocks = new_blocks;
        self.direction.move_by(movement);
        self.center.translate_by(movement);
        Ok(())
    }

    /// Returns an Option eventually containing a Tetromino if its starting position is empty.
    pub(in crate::app::player) fn new(kind: TetrominoKind, grid: &TetrisGrid) -> Option<Tetromino> {
        let positions = kind.get_initial_position();
        let color = kind.get_color();
        for i in 1..5 {
            if grid.matrix[positions[2 * i + 1] as usize][positions[2 * i] as usize].is_some() {
                return None;
            }
        }
        Some(Tetromino {
            kind,
            center: Position::new(positions[0], positions[1]),
            blocks: [
                Block::new(color, positions[2], positions[3]),
                Block::new(color, positions[4], positions[5]),
                Block::new(color, positions[6], positions[7]),
                Block::new(color, positions[8], positions[9]),
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
            center: Position::new(positions[0], positions[1]),
            blocks: [
                Block::new(color, positions[2], positions[3]),
                Block::new(color, positions[4], positions[5]),
                Block::new(color, positions[6], positions[7]),
                Block::new(color, positions[8], positions[9]),
            ],
            direction: Direction::default(),
            is_ghost: false,
        }
    }

    // TODO can't this cause a collision??
    /// Resets the Tetromino at its starting position.
    pub fn reset_position(&mut self) {
        let positions = self.kind.get_initial_position();
        let color = self.kind.get_color();
        self.center = Position::new(positions[0], positions[1]);
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
            center: Position::default(),
            blocks: [Block::default(); 4],
            direction: Direction::default(),
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

    pub(super) fn blocks(&self) -> &[Block] {
        &self.blocks
    }
}
