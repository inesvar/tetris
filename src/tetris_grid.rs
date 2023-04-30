use graphics::{Context, rectangle};
use graphics::math::margin_rectangle;
use graphics::types::{Color, Scalar};
use opengl_graphics::GlGraphics;
use crate::block::Block;
use crate::settings::{BLOCK_SHRINK, BLOCK_SIZE};

pub enum GridCell {
    Empty,
    Block(Block)
}

pub struct TetrisGrid {
    pub width: i8,
    pub height: i8,
    pub rows: Vec<Vec<GridCell>>
}

impl TetrisGrid {
    pub fn new(width: i8, height: i8) -> TetrisGrid {
        let mut rows = Vec::with_capacity(height as usize);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width as usize);
            for _ in 0..width {
                row.push(GridCell::Empty);
            }
            rows.push(row);
        }
        TetrisGrid {
            width,
            height,
            rows
        }
    }

    pub fn new_random(width: i8, height: i8, probability: f32) -> TetrisGrid {
        let mut rows = Vec::with_capacity(height as usize);
        for y in 0..height {
            let mut row = Vec::with_capacity(width as usize);
            for x in 0..width {
                if rand::random::<f32>() < probability {
                    row.push(GridCell::Block(Block::new([1.0, 0.0, 0.0, 1.0], x, y)));
                } else {
                    row.push(GridCell::Empty);
                }
            }
            rows.push(row);
        }
        TetrisGrid {
            width,
            height,
            rows
        }
    }

    pub fn render(&self, ctx: &Context, gl: &mut GlGraphics) {
        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                // draw white square outline
                let mut square = rectangle::square(
                    x as Scalar * BLOCK_SIZE,
                    y as Scalar * BLOCK_SIZE,
                    BLOCK_SIZE
                );

                square = margin_rectangle(square, BLOCK_SHRINK);

                rectangle([0.1, 0.1, 0.1, 1.0], square, ctx.transform, gl);

                match cell {
                    GridCell::Empty => {},
                    GridCell::Block(block) => block.render(ctx, gl)
                }
            }
        }
    }
}