use crate::assets::{Assets, TetrisColor};
use crate::block::Block;
use crate::point::Transformable;
use crate::settings::{BLOCK_SIZE, GRID_COLOR, GRID_THICKNESS};
use crate::tetromino::Tetromino;
use graphics::math::Matrix2d;
use graphics::types::{Rectangle, Scalar};
use graphics::Transformed;
use graphics::{rectangle, Context};
use opengl_graphics::GlGraphics;
use piston_window::RenderArgs;

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
