use crate::assets::Assets;
use crate::assets::TetrisColor;
use crate::point::{Point, Transformable};
use crate::settings::BLOCK_SIZE;
use graphics::math::{Matrix2d, Scalar};
use graphics::{rectangle, DrawState, Image};
use opengl_graphics::GlGraphics;

#[derive(Clone, Copy)]
pub struct Block {
    color: TetrisColor,
    pub position: Point,
}

impl Block {
    pub fn new(color: TetrisColor, x: i8, y: i8) -> Self {
        Block {
            color,
            position: Point::new(x, y),
        }
    }

    pub fn set_position(mut self, point: Point) {
        self.position = point;
    }

    pub fn render(
        &self,
        transform: Matrix2d,
        draw_state: &DrawState,
        gl: &mut GlGraphics,
        assets: &Assets,
    ) {
        let dims = rectangle::square(
            self.position.x as Scalar * BLOCK_SIZE,
            self.position.y as Scalar * BLOCK_SIZE,
            BLOCK_SIZE,
        );

        Image::new().rect(dims).draw(
            assets.texture_from_tetris_color(&self.color),
            draw_state,
            transform,
            gl,
        );
    }
}

impl Transformable for Block {
    /* methode to new

    fn go_down(mut self) -> Self {
        self.position = self.position.go_down();
        self
    }*/

    fn go_down(&mut self) {
        self.position.go_down();
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
    fn fall(&self, matrix: &Vec<Vec<Option<Block>>>) -> Result<Block, ()>;
    fn right(&self, matrix: &Vec<Vec<Option<Block>>>) -> Result<Block, ()>;
    fn left(&self, matrix: &Vec<Vec<Option<Block>>>) -> Result<Block, ()>;
    fn turn_clockwise(&self, other: &Point, matrix: &Vec<Vec<Option<Block>>>) -> Result<Block, ()>;
    fn turn_counterclockwise(&self, other: &Point, matrix: &Vec<Vec<Option<Block>>>) -> Result<Block, ()>;
}

impl Collision for Block {
    fn fall(&self, matrix: &Vec<Vec<Option<Block>>>) -> Result<Block, ()> {
        let mut copy = *self;
        copy.go_down();
        if copy.position.y as usize > matrix.len() - 1 {
            Err(())
        } else {
            match matrix[copy.position.y as usize][copy.position.x as usize] {
                Some(_) => Err(()),
                None => {
                    Ok(copy)
                }
            }
        }
    }

    fn left(&self, matrix: &Vec<Vec<Option<Block>>>) -> Result<Block, ()> {
        let mut copy = *self;
        copy.go_left();
        if copy.position.x < 0 {
            Err(())
        } else {
            match matrix[copy.position.y as usize][copy.position.x as usize] {
                Some(_) => Err(()),
                None => {
                    Ok(copy)
                }
            }
        }
    }

    fn right(&self, matrix: &Vec<Vec<Option<Block>>>) -> Result<Block, ()> {
        let mut copy = *self;
        copy.go_right();
        if copy.position.x as usize > matrix[0].len() - 1 {
            Err(())
        } else {
            match matrix[copy.position.y as usize][copy.position.x as usize] {
                Some(_) => Err(()),
                None => {
                    Ok(copy)
                }
            }
        }
    }

    fn turn_clockwise(&self, other: &Point, matrix: &Vec<Vec<Option<Block>>>) -> Result<Block, ()> {
        let mut copy = *self;
        copy.rotate_clockwise(other);
        match (copy.position.x, copy.position.y) {
            (x, y) if x < 0 || y < 0 => {
                return Err(());
            }
            (x, y) if x as usize >= matrix[0].len() - 1 || y as usize >= matrix.len() - 1 => {
                return Err(());
            }
            _ => {}
        }
        match matrix[copy.position.y as usize][copy.position.x as usize] {
            Some(_) => Err(()),
            None => {
                Ok(copy)
            }
        }
    }

    fn turn_counterclockwise(&self, other: &Point, matrix: &Vec<Vec<Option<Block>>>) -> Result<Block, ()> {
        let mut copy = *self;
        copy.rotate_counterclockwise(other);
        match (copy.position.x, copy.position.y) {
            (x, y) if x < 0 || y < 0 => {
                return Err(());
            }
            (x, y) if x as usize >= matrix[0].len() - 1 || y as usize >= matrix.len() - 1 => {
                return Err(());
            }
            _ => {}
        }
        match matrix[copy.position.y as usize][copy.position.x as usize] {
            Some(_) => Err(()),
            None => {
                Ok(copy)
            }
        }
    }
}
