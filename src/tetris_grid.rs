use graphics::{Context, rectangle, color};
use graphics::math::{Matrix2d};
use graphics::types::{Rectangle, Scalar};
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston_window::RenderArgs;
use crate::assets::{Assets, TetrisColor};
use crate::block::Block;
use crate::point::Transformable;
use crate::settings::{BLOCK_SIZE, GRID_THICKNESS, GRID_COLOR};
use crate::tetromino::Tetromino;

pub struct TetrisGrid {
    pub nb_columns: i8,
    pub nb_rows: i8,
    pub rows: Vec<Vec<Option<Block>>>,
    pub line_sum: Vec<u8>,
    pub total_width: f64,
    pub total_height: f64,
    pub visible_width: f64,
    pub visible_height: f64,
    pub transform: Matrix2d<f64>,
    pub nb_lines_cleared_last_frame: u8,
}

impl TetrisGrid {
    pub fn new(nb_columns: i8, nb_rows: i8) -> TetrisGrid {
        let mut rows = Vec::with_capacity(nb_rows as usize);
        for _ in 0..nb_rows {
            rows.push(vec![None; nb_columns as usize]);
        }

        let line_sum = vec![0; nb_rows as usize];
        TetrisGrid {
            nb_columns,
            nb_rows,
            rows,
            line_sum,
            total_width: nb_columns as f64 * BLOCK_SIZE,
            total_height: nb_rows as f64 * BLOCK_SIZE,
            visible_width: nb_columns as f64 * BLOCK_SIZE,
            visible_height: (nb_rows - 2) as f64 * BLOCK_SIZE,
            transform: Matrix2d::default(),
            nb_lines_cleared_last_frame: 0,
        }
    }

    pub fn freeze_tetromino(&mut self, tetromino: &mut Tetromino) {
        let mut blocks = tetromino.split();
        for block in &mut blocks {
            self.rows[block.position.y as usize][block.position.x as usize] = Some(*block);
            self.line_sum[block.position.y as usize] += 1;
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
            total_width: nb_columns as f64 * BLOCK_SIZE,
            total_height: nb_rows as f64 * BLOCK_SIZE,
            visible_width: nb_columns as f64 * BLOCK_SIZE,
            visible_height: (nb_rows - 2) as f64 * BLOCK_SIZE,
            transform: Matrix2d::default(),
            nb_lines_cleared_last_frame: 0,
        }
    }

    pub fn update(&mut self) {
        self.nb_lines_cleared_last_frame = 0;

        for y in 0..self.nb_rows {
            if self.line_sum[y as usize] == self.nb_columns as u8 {
                self.rows.remove(y as usize);
                self.rows.insert(0, vec![None; self.nb_columns as usize]);

                self.line_sum.remove(y as usize);
                self.line_sum.insert(0, 0);

                // move all block above down
                for y2 in 0..=y {
                    for elm in self.rows[y2 as usize].iter_mut() {
                        if let Some(block) = elm {
                            block.go_down();
                        }
                    }
                }

                self.nb_lines_cleared_last_frame += 1;
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs, ctx: &Context, gl: &mut GlGraphics, assets: &Assets) {
        self.transform = ctx.transform.trans(
            args.window_size[0] / 2.0 - self.total_width / 2.0,
            args.window_size[1] / 2.0 - self.total_height / 2.0,
        );

        let empty_dims: Rectangle = [0.0, self.total_height - self.visible_height, self.visible_width, self.visible_height];
        rectangle([0.1, 0.1, 0.1, 1.0], empty_dims, self.transform, gl);

        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if y > 1 {
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

    pub fn print_grid(matrix: &Vec<Vec<Option<Block>>>) {
        for row in matrix.iter() {
            for cell in row.iter() {
                match cell {
                    Some(_) => print!("X"),
                    None => print!("_"),
                };
            }
            println!();
        }
    }

    pub fn null(&mut self) {
        for row in self.rows.iter_mut() {
            for cell in row.iter_mut() {
                *cell = None;
            }
        }
    }
}