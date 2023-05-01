use graphics::{Context, rectangle};
use graphics::math::{margin_rectangle, Matrix2d, Scalar};
use crate::point::{Point, Transformable};
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use crate::settings::{BLOCK_SHRINK, BLOCK_SIZE};

pub struct Block {
    color: Color,
    position: Point<i8>,
}

impl Block {
    pub fn new(color: Color, x: i8, y: i8) -> Self {
        Block {
            color,
            position: Point::new(x, y),
        }
    }

    pub fn set_position(mut self, position: Point<i8>) -> Self {
        self.position = position;
        self
    }

    pub fn render(&self, transform: Matrix2d, gl: &mut GlGraphics) {
        let mut dims = rectangle::square(
            self.position.x as Scalar * BLOCK_SIZE,
            self.position.y as Scalar * BLOCK_SIZE,
            BLOCK_SIZE,
        );
        dims = margin_rectangle(dims, BLOCK_SHRINK);

        rectangle(self.color, dims, transform, gl);
    }
}

impl Transformable for Block {
    fn down(self) {
        self.position.down();
    }

    /* methode sans enplace
    
    fn right(mut self) -> Self {
        self.position = self.position.right();
        self
    }*/

    fn left(self) {
        self.position.left();
    }

    fn right(self) {
        self.position.right();
    }

    fn turn_clockwise(self) {
        self.position.turn_clockwise();
    }

    fn turn_counterclockwise(self) {
        self.position.turn_counterclockwise();
    }
}
