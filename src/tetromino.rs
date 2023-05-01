use graphics::Context;
use graphics::types::{Matrix2d};
use opengl_graphics::GlGraphics;
use crate::point::{Point, Transformable};

use crate::{block::Block};
use crate::assets::TetrisColor;
use crate::assets::Assets;

pub struct Tetromino {
    color: TetrisColor,
    center: Point,
    blocks: [Block; 4],
    rotation_status: Rotation,
}

pub enum Rotation {
    R0,
    R1,
    R2,
    R3,
}

impl Tetromino {
    pub fn new(color: TetrisColor, positions: &mut [i8]) -> Self {
        for i in 1..5 {
            positions[2*i] += positions[0];
            positions[2*i + 1] += positions[1];
        }
        Tetromino {
            color,
            center: Point::new(positions[0], positions[1]),
            blocks: [Block::new(color, positions[2], positions[3]), 
                            Block::new(color, positions[4], positions[5]), 
                            Block::new(color, positions[6], positions[7]),
                            Block::new(color, positions[8], positions[9])],
            rotation_status: Rotation::R0,
        }
    }
}

/* TRANSFORMABLE METHODS */
impl Tetromino {
    pub fn down(&mut self) {
        for i in 0..4 {
            self.blocks[i].down();
        }
        self.center.down();
    }

    pub fn left(&mut self) {
        for i in 0..4 {
            self.blocks[i].left();
        }
        self.center.left();
    }

    pub fn right(&mut self) {
        for i in 0..4 {
            self.blocks[i].right();
        }
        self.center.right();
    }

    pub fn turn_clockwise(&mut self) {
        for i in 0..4 {
            self.blocks[i].turn_clockwise(&self.center);
        }
    }

    pub fn turn_counterclockwise(&mut self) {
        for i in 0..4 {
            self.blocks[i].turn_counterclockwise(&self.center);
        }
    }
}

impl Tetromino {
    pub fn render(&self, transform: Matrix2d, ctx: &Context, gl: &mut GlGraphics, assets: &Assets) {
        for i in 0..4 {
            self.blocks[i].render(transform, &ctx.draw_state, gl, assets);
        }
    }
}

impl Tetromino {
    pub fn split(self) -> [Block; 4] {
        self.blocks
    }
}
