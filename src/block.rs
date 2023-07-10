use serde::{Deserialize, Serialize};

use crate::assets::TetrisColor;
use crate::point::{Point, Transformable};
use crate::translate_rotate::TranslateRotate;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Block {
    pub(crate) color: TetrisColor,
    pub position: Point,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            color: TetrisColor::Yellow,
            position: Point::default(),
        }
    }
}

impl Block {
    pub fn new(color: TetrisColor, x: i8, y: i8) -> Self {
        Block {
            color,
            position: Point::new(x, y),
        }
    }

    pub fn translation(self, point: Point) -> Self {
        Block {
            color: self.color,
            position: self.position + point,
        }
    }
}

impl Transformable for Block {
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

pub trait Collision {
    fn move_to(
        &self,
        matrix: &[Vec<Option<Block>>],
        movement: &TranslateRotate,
    ) -> Result<Block, ()>;
}

impl Collision for Block {
    fn move_to(
        &self,
        matrix: &[Vec<Option<Block>>],
        movement: &TranslateRotate,
    ) -> Result<Block, ()> {
        let mut copy = self.translation(movement.translation);
        match movement.rotation {
            1 => {
                copy.rotate_clockwise(&movement.center.unwrap());
            }
            -1 => {
                copy.rotate_counterclockwise(&movement.center.unwrap());
            }
            _ => {}
        }
        match (copy.position.x, copy.position.y) {
            (x, y) if x < 0 || y < 0 => {
                return Err(());
            }
            (x, y) if x as usize >= matrix[0].len() || y as usize >= matrix.len() => {
                return Err(());
            }
            _ => {}
        }
        match matrix[copy.position.y as usize][copy.position.x as usize] {
            Some(_) => Err(()),
            None => Ok(copy),
        }
    }
}
