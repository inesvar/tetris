use graphics::{Context, rectangle};
use graphics::math::{Matrix2d};
use graphics::types::{Rectangle, Scalar};
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston_window::RenderArgs;
use crate::assets::{Assets, TetrisColor};
use crate::block::Block;
use crate::settings::{BLOCK_SIZE, GRID_THICKNESS, GRID_COLOR};
use crate::tetromino::Tetromino;

pub struct TetrisGrid {
    pub nb_columns: i8,
    pub nb_rows: i8,
    pub rows: Vec<Vec<Option<Block>>>,
    pub line_sum : Vec<u8>,
    pub width: f64,
    pub height: f64,
    pub transform: Matrix2d<f64>
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
            height: nb_rows as f64 * BLOCK_SIZE,
            transform: Matrix2d::default()
        }
    }

    pub fn freeze_tetromino(&mut self, tetromino: Tetromino) {
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
            height: nb_rows as f64 * BLOCK_SIZE,
            transform: Matrix2d::default()
        }
    }

    pub fn render(&mut self, args: &RenderArgs, ctx: &Context, gl: &mut GlGraphics, assets: &Assets) {
        self.transform = ctx.transform.trans(
            args.window_size[0] / 2.0 - self.width / 2.0,
            args.window_size[1] / 2.0 - self.height / 2.0
        );

        let empty_dims: Rectangle = [0.0, 0.0, self.width, self.height];
        rectangle([0.1, 0.1, 0.1, 1.0], empty_dims, self.transform, gl);

        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let outline_rect = graphics::Rectangle::new_border(GRID_COLOR, GRID_THICKNESS);
                let outline_dims = rectangle::square(x as Scalar * BLOCK_SIZE, y as Scalar * BLOCK_SIZE, BLOCK_SIZE);
                outline_rect.draw(outline_dims, &ctx.draw_state, self.transform, gl);

                match cell {
                    Some(block) => block.render(self.transform, &ctx.draw_state, gl, assets),
                    None => {}
                }
            }
        }
    }
}