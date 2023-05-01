use graphics::{color, Context, draw_state, rectangle};
use graphics::math::margin_rectangle;
use graphics::types::{Color, Rectangle, Scalar};
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston_window::RenderArgs;
use crate::assets::{Assets, TetrisColor};
use crate::block::Block;
use crate::settings::{BLOCK_SIZE, DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT};
use crate::tetromino::Tetromino;

pub struct TetrisGrid {
    pub nb_columns: i8,
    pub nb_rows: i8,
    pub rows: Vec<Vec<Option<Block>>>,
    pub line_sum : Vec<u8>,
    pub width: f64,
    pub height: f64
}

impl TetrisGrid {
    pub fn new(nb_columns: i8, nb_rows: i8) -> TetrisGrid {
        let mut rows = Vec::with_capacity(nb_rows as usize);
        for _ in 0..nb_rows {
            rows.push(vec![None; nb_columns as usize]);
        }
        
        let line_sum = vec![0; nb_columns as usize];
        TetrisGrid {
            nb_columns,
            nb_rows,
            rows,
            line_sum: line_sum,
            width: nb_columns as f64 * BLOCK_SIZE,
            height: nb_rows as f64 * BLOCK_SIZE
        }
    }

    pub fn add_tetromino(&mut self, tetromino: Tetromino) {
        let blocks = tetromino.split();
        for i in 0..4 {
            self.rows[blocks[i].position.y as usize][blocks[i].position.x as usize] = Some(blocks[i]);
            self.line_sum[blocks[i].position.y as usize] += 1;
        }
    }

    pub fn new_random(nb_columns: i8, nb_rows: i8, probability: f32) -> TetrisGrid {
        let mut rows = Vec::with_capacity(nb_rows as usize);
        for y in 0..nb_rows {
            let mut row = Vec::with_capacity(nb_columns as usize);
            for x in 0..nb_columns {
                if rand::random::<f32>() < probability {
                    row.push(Some(Block::new(TetrisColor::ORANGE, x, y)));
                } else {
                    row.push(None);
                }
            }
            rows.push(row);
        }
        let line_sum = vec![0; nb_columns as usize];
        TetrisGrid {
            nb_columns,
            nb_rows,
            rows,
            line_sum,
            width: nb_columns as f64 * BLOCK_SIZE,
            height: nb_rows as f64 * BLOCK_SIZE
        }
    }

    pub fn render(&self, args: &RenderArgs, ctx: &Context, gl: &mut GlGraphics, assets: &Assets) {
        let grid_transform = ctx.transform.trans(
            args.window_size[0] / 2.0 - self.width / 2.0,
            args.window_size[1] / 2.0 - self.height / 2.0
        );
        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let outline_rect = graphics::Rectangle::new_border(color::GRAY, 0.2);
                let outline_dims = rectangle::square(x as Scalar * BLOCK_SIZE, y as Scalar * BLOCK_SIZE, BLOCK_SIZE);
                outline_rect.draw(outline_dims, &ctx.draw_state, grid_transform, gl);

                match cell {
                    None => {
                        let empty_dims = rectangle::square(
                            x as Scalar * BLOCK_SIZE,
                            y as Scalar * BLOCK_SIZE,
                            BLOCK_SIZE
                        );
                        rectangle([0.1, 0.1, 0.1, 1.0], empty_dims, grid_transform, gl);
                    },
                    Some(block) => block.render(grid_transform, &ctx.draw_state, gl, assets)
                }
            }
        }
    }
}