use crate::point::{Point, Transformable};
use crate::assets::TetrisColor;
use crate::settings::{BLOCK_SIZE};
use graphics::math::{margin_rectangle, Matrix2d, Scalar};
use graphics::types::Color;
use graphics::{rectangle, Context, DrawState, Image};
use opengl_graphics::GlGraphics;
use crate::assets::Assets;

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

    /*pub fn set_position(mut self, x: i8, y: i8) {
        Point::set(&mut self.position, x, y);
    }*/

    pub fn render(&self, transform: Matrix2d, draw_state: &DrawState, gl: &mut GlGraphics, assets: &Assets) {
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

    fn down(mut self) -> Self {
        self.position = self.position.down();
        self
    }*/

    fn down(&mut self) {
        self.position.down();
    }

    fn left(&mut self) {
        self.position.left();
    }

    fn right(&mut self) {
        self.position.right();
    }

    fn turn_clockwise(&mut self, other: &Point) {
        self.position.turn_clockwise(other);
    }

    fn turn_counterclockwise(&mut self, other: &Point) {
        self.position.turn_counterclockwise(other);
    }
}
