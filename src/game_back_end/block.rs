//! Defines the basics for collisions in the grid.
use super::point::{Point, Transform};
use super::translation_rotation::{Rotation, TranslationRotation};
use crate::assets::TetrisColor;
use serde::{Deserialize, Serialize};

/// Coloured tetris Block in a finite wrap-around 2D grid.
///
/// Notably implements **Collision** trait.
///
/// A block can be rendered and implements collisions,
/// but it can also be moved without using information
/// about its surroudings through the Transform trait.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Block {
    pub position: Point,
    pub color: TetrisColor,
}

/// Moves blocks in the grid in such a way that :
/// - blocks stay in the playing field
/// - blocks can't overlap
///
/// ## Uses
/// - anytime a Tetromino moves, its *blocks* rely on Collision
///
/// ## Functions
/// - **move_to**(matrix: &\[Vec<Option<Block>>\], movement: &TranslationRotation)
///
/// returns either a block with the resulting coordinates or Err(())
pub trait Collision {
    fn move_to(
        &self,
        matrix: &[Vec<Option<Block>>],
        movement: &TranslationRotation,
    ) -> Result<Block, ()>;
}

impl Block {
    pub fn new(color: TetrisColor, x: i8, y: i8) -> Self {
        Block {
            position: Point::new(x, y),
            color,
        }
    }

    pub fn translation(self, other: Point) -> Self {
        Block {
            position: self.position + other,
            color: self.color,
        }
    }

    fn check_inside_grid(point: &Point, width: usize, height: usize) -> Result<(), ()> {
        if point.x < 0 || point.y < 0 {
            return Err(());
        }
        if point.x as usize >= width || point.y as usize >= height {
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

impl Transform for Block {
    fn go_down(&mut self) {
        self.position.go_down();
    }

    fn go_up(&mut self) {
        self.position.go_up();
    }

    fn go_left(&mut self) {
        self.position.go_left();
    }

    fn go_right(&mut self) {
        self.position.go_right();
    }

    fn rotate_clockwise(&mut self, other: &Point) {
        self.position.rotate_clockwise(other);
    }

    fn rotate_counterclockwise(&mut self, other: &Point) {
        self.position.rotate_counterclockwise(other);
    }
}
impl Collision for Block {
    fn move_to(
        &self,
        matrix: &[Vec<Option<Block>>],
        movement: &TranslationRotation,
    ) -> Result<Block, ()> {
        // Apply the TranslationRotation
        let mut copy = self.translation(movement.translation);
        match movement.rotation {
            Rotation::Clockwise(center) => {
                copy.rotate_clockwise(&center);
            }
            Rotation::Counterclockwise(center) => {
                copy.rotate_counterclockwise(&center);
            }
            _ => {}
        }
        // Check if the block is still inside the grid
        Self::check_inside_grid(&copy.position, matrix[0].len(), matrix.len())?;
        // Check if the block is not on another one
        match matrix[copy.position.y as usize][copy.position.x as usize] {
            Some(_) => Err(()),
            None => Ok(copy),
        }
    }
}
