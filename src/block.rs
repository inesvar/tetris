use crate::assets::Assets;
use crate::assets::TetrisColor;
use crate::point::{Point, Transformable};
use crate::settings::BLOCK_SIZE;
use graphics::math::{margin_rectangle, Matrix2d, Scalar};
use graphics::types::Color;
use graphics::{rectangle, Context, DrawState, Image};
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

pub enum NewBlock {
    Error,
    Success(Block),
}

pub trait Collision {
    fn fall(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> NewBlock;
    fn right(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> NewBlock;
    fn left(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> NewBlock;
    fn turn_clockwise(&mut self, other: &Point, matrix: &Vec<Vec<Option<Block>>>) -> NewBlock;
    fn turn_counterclockwise(
        &mut self,
        other: &Point,
        matrix: &Vec<Vec<Option<Block>>>,
    ) -> NewBlock;
}

impl Collision for Block {
    fn fall(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> NewBlock {
        if self.position.y <= 0 {
            NewBlock::Error
        } else {
            match matrix[(self.position.y - 1) as usize][self.position.x as usize] {
                Some(_) => NewBlock::Error,
                None => {
                    self.go_down();
                    NewBlock::Success(*self)
                }
            }
        }
    }

    fn left(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> NewBlock {
        if self.position.x <= 0 {
            NewBlock::Error
        } else {
            match matrix[self.position.y as usize][(self.position.x - 1) as usize] {
                Some(_) => NewBlock::Error,
                None => {
                    self.go_left();
                    NewBlock::Success(*self)
                }
            }
        }
    }

    fn right(&mut self, matrix: &Vec<Vec<Option<Block>>>) -> NewBlock {
        if self.position.x as usize >= matrix[0].len() - 1 {
            NewBlock::Error
        } else {
            match matrix[self.position.y as usize][(self.position.x - 1) as usize] {
                Some(_) => NewBlock::Error,
                None => {
                    self.go_right();
                    NewBlock::Success(*self)
                }
            }
        }
    }

    fn turn_clockwise(&mut self, other: &Point, matrix: &Vec<Vec<Option<Block>>>) -> NewBlock {
        let mut copy = self.position;
        copy.rotate_clockwise(other);
        match (copy.x, copy.y) {
            (x, y) if x < 0 || y < 0 => {
                return NewBlock::Error;
            }
            (x, y) if x as usize >= matrix[0].len() - 1 || y as usize >= matrix.len() - 1 => {
                return NewBlock::Error;
            }
            _ => {}
        }
        match matrix[copy.y as usize][copy.x as usize] {
            Some(_) => NewBlock::Error,
            None => {
                self.set_position(copy);
                NewBlock::Success(*self)
            }
        }
    }

    fn turn_counterclockwise(&mut self, other: &Point, matrix: &Vec<Vec<Option<Block>>>) -> NewBlock {
        let mut copy = self.position;
        copy.rotate_counterclockwise(other);
        match (copy.x, copy.y) {
            (x, y) if x < 0 || y < 0 => {
                return NewBlock::Error;
            }
            (x, y) if x as usize >= matrix[0].len() - 1 || y as usize >= matrix.len() - 1 => {
                return NewBlock::Error;
            }
            _ => {}
        }
        match matrix[copy.y as usize][copy.x as usize] {
            Some(_) => NewBlock::Error,
            None => {
                self.set_position(copy);
                NewBlock::Success(*self)
            }
        }
    }
}
