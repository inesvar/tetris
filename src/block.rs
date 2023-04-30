use graphics::{Context, rectangle};
use graphics::math::margin_rectangle;
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

    pub fn render(&self, ctx: &Context, gl: &mut GlGraphics) {
        let mut square = rectangle::square(
            self.position.x as f64 * BLOCK_SIZE,
            self.position.y as f64 * BLOCK_SIZE,
            BLOCK_SIZE,
        );

        square = margin_rectangle(square, BLOCK_SHRINK);

        rectangle(self.color, square, ctx.transform, gl);
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
