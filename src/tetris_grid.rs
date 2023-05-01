use graphics::{color, Context, draw_state, rectangle};
use graphics::math::margin_rectangle;
use graphics::types::{Color, Rectangle, Scalar};
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use crate::block::Block;
use crate::settings::{BLOCK_SHRINK, BLOCK_SIZE, WINDOW_WIDTH, WINDOW_HEIGHT};

pub struct TetrisGrid {
    pub nb_columns: i8,
    pub nb_rows: i8,
    pub rows: Vec<Vec<Option<Block>>>,
    pub width: f64,
    pub height: f64
}

impl TetrisGrid {
    pub fn new(nb_columns: i8, nb_rows: i8) -> TetrisGrid {
        let mut rows = Vec::with_capacity(nb_rows as usize);
        for _ in 0..nb_rows {
            let mut row = Vec::with_capacity(nb_columns as usize);
            for _ in 0..nb_columns {
                row.push(None);
            }
            rows.push(row);
        }
        TetrisGrid {
            nb_columns,
            nb_rows,
            rows,
            width: nb_columns as f64 * BLOCK_SIZE,
            height: nb_rows as f64 * BLOCK_SIZE
        }
    }

    pub fn new_random(nb_columns: i8, nb_rows: i8, probability: f32) -> TetrisGrid {
        let mut rows = Vec::with_capacity(nb_rows as usize);
        for y in 0..nb_rows {
            let mut row = Vec::with_capacity(nb_columns as usize);
            for x in 0..nb_columns {
                if rand::random::<f32>() < probability {
                    row.push(Some(Block::new([1.0, 0.0, 0.0, 1.0], x, y)));
                } else {
                    row.push(None);
                }
            }
            rows.push(row);
        }
        TetrisGrid {
            nb_columns,
            nb_rows,
            rows,
            width: nb_columns as f64 * BLOCK_SIZE,
            height: nb_rows as f64 * BLOCK_SIZE
        }
    }

    pub fn render(&self, ctx: &Context, gl: &mut GlGraphics) {
        let grid_transform = ctx.transform.trans(
            WINDOW_WIDTH as f64 / 2.0 - self.width / 2.0,
            WINDOW_HEIGHT as f64 / 2.0 - self.height / 2.0
        );
        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let outline_rect = graphics::Rectangle::new_border(color::GRAY, 0.2);
                let outline_dims = rectangle::square(x as Scalar * BLOCK_SIZE, y as Scalar * BLOCK_SIZE, BLOCK_SIZE);
                outline_rect.draw(outline_dims, &ctx.draw_state, grid_transform, gl);

                match cell {
                    None => {
                        let mut empty_dims = rectangle::square(
                            x as Scalar * BLOCK_SIZE,
                            y as Scalar * BLOCK_SIZE,
                            BLOCK_SIZE
                        );
                        empty_dims = margin_rectangle(empty_dims, BLOCK_SHRINK);
                        rectangle([0.1, 0.1, 0.1, 1.0], empty_dims, grid_transform, gl);
                    },
                    Some(block) => block.render(grid_transform, gl)
                }
            }
        }
    }
}