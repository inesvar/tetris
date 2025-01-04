//! Defines a tetris block and the useful functions to move it inside a tetris grid.
use super::{
    point::{Point, TetrisMoves},
    translation_rotation::Rotation,
    GridMatrix, TranslationRotation,
};
use crate::assets::TetrisColor;
use serde::{Deserialize, Serialize};

/// Coloured tetris block in a finite 2D grid.
///
/// A block can move inside a grid, constrained by the bounds of the grid and other blocks, through [Collision].
/// However it can also move without using information
/// about its surroudings through [TetrisMoves].
/// A block has a [render()](Block::render()).
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Block {
    position: Point,
    pub(super) color: TetrisColor,
}

/// Moves a block inside the grid, eventually handling the collision with other blocks in the matrix.
///
/// The block can never leave the matrix.
///
/// ## Uses
/// - anytime a [Tetromino](super::Tetromino) moves
pub(super) trait Collision {
    /// Applies a given *movement* to a block in a given *matrix*.
    fn move_to(&self, matrix: &GridMatrix, movement: &TranslationRotation) -> Result<Block, ()>;
}

impl Block {
    pub(super) fn new(color: TetrisColor, x: i8, y: i8) -> Self {
        Block {
            position: Point::new(x, y),
            color,
        }
    }

    fn translation_by(self, other: &TranslationRotation) -> Self {
        Block {
            position: self.position + other.translation,
            color: self.color,
        }
    }

    fn check_inside_grid(point: &Point, width: usize, height: usize) -> Result<(), ()> {
        if point.x() < 0 || point.y() < 0 {
            return Err(());
        }
        if point.x() as usize >= width || point.y() as usize >= height {
            return Err(());
        }
        Ok(())
    }
}

impl Default for Block {
    fn default() -> Self {
        Block {
            position: Point::default(),
            color: TetrisColor::Yellow,
        }
    }
}

impl Collision for Block {
    fn move_to(&self, matrix: &GridMatrix, movement: &TranslationRotation) -> Result<Block, ()> {
        // Apply the TranslationRotation
        let mut copy = self.translation_by(movement);
        match movement.rotation {
            Rotation::Clockwise(center) => {
                copy.turn_clockwise_around(&center);
            }
            Rotation::Counterclockwise(center) => {
                copy.turn_counterclockwise_around(&center);
            }
            _ => {}
        }
        // Check if the block is still inside the grid
        Self::check_inside_grid(&copy.position, matrix[0].len(), matrix.len())?;
        // Check if the block is not on another one
        match matrix[copy.y() as usize][copy.x() as usize] {
            Some(_) => Err(()),
            None => Ok(copy),
        }
    }
}

impl TetrisMoves for Block {
    fn x(&self) -> i8 {
        self.position.x()
    }

    fn y(&self) -> i8 {
        self.position.y()
    }

    fn go_down(&mut self) {
        self.position.go_down();
    }

    fn go_left(&mut self) {
        self.position.go_left();
    }

    fn go_right(&mut self) {
        self.position.go_right();
    }

    fn turn_clockwise_around(&mut self, other: &Point) {
        self.position.turn_clockwise_around(other);
    }

    fn turn_counterclockwise_around(&mut self, other: &Point) {
        self.position.turn_counterclockwise_around(other);
    }
}
