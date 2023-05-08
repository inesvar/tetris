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

    pub fn translation(self, point: Point) -> Self {
        Block {
            color: self.color,
            position: self.position + point,
        }
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
    fn move_to(
        &self,
        matrix: &Vec<Vec<Option<Block>>>,
        movement: &TranslateRotate,
    ) -> Result<Block, ()>;
}

impl Collision for Block {
    fn move_to(
        &self,
        matrix: &Vec<Vec<Option<Block>>>,
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

pub struct TranslateRotate {
    translation: Point,
    rotation: i8,
    center: Option<Point>,
}

impl TranslateRotate {
    pub fn new(translation: Point, rotation: i8, center: &Point) -> Self {
        let center = *center + translation;
        TranslateRotate {
            translation,
            rotation,
            center: Some(center),
        }
    }
}
