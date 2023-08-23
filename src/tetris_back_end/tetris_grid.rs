//! Defines the implementation of a [TetrisGrid](super::TetrisGrid).
use super::{block::Block, point::Transform, TetrisGrid, Tetromino};
use crate::assets::TetrisColor;
use crate::settings::BLOCK_SIZE;
use graphics::types::Matrix2d;
use rand::Rng;

impl TetrisGrid {
    pub fn new(x: f64, y: f64, nb_columns: u32, nb_rows: u32) -> TetrisGrid {
        let mut rows = Vec::with_capacity(nb_rows as usize);
        for _ in 0..nb_rows {
            rows.push(vec![None; nb_columns as usize]);
        }

        let line_sum = vec![0; nb_rows as usize];
        TetrisGrid {
            x,
            y,
            nb_columns,
            nb_rows,
            rows,
            line_sum,
            total_width: nb_columns as f64 * BLOCK_SIZE,
            total_height: nb_rows as f64 * BLOCK_SIZE,
            visible_width: nb_columns as f64 * BLOCK_SIZE,
            visible_height: (nb_rows - 2) as f64 * BLOCK_SIZE,
            transform: Matrix2d::default(),
        }
    }

    /// Push the Tetromino into the grid and return the number of lines completed.
    pub fn freeze_tetromino(&mut self, tetromino: &mut Tetromino) -> Option<u64> {
        let mut game_over = true;
        let mut blocks = tetromino.split();
        for block in &mut blocks {
            self.rows[block.position.y as usize][block.position.x as usize] = Some(*block);
            self.line_sum[block.position.y as usize] += 1;
            // if there's a block below the top of the visible grid, continue playing
            if block.position.y as usize > 1 {
                game_over = false;
            }
        }
        if game_over {
            return None;
        }
        let mut score = 0;
        for y in 0..self.nb_rows {
            if self.line_sum[y as usize] == self.nb_columns as u8 {
                // TODO neatly separate this in a private function
                self.rows.remove(y as usize);
                self.rows.insert(0, vec![None; self.nb_columns as usize]);

                self.line_sum.remove(y as usize);
                self.line_sum.insert(0, 0);

                // move all block above down
                for y2 in 0..=y {
                    for block in self.rows[y2 as usize].iter_mut().flatten() {
                        block.go_down();
                    }
                }

                score += 1;
            }
        }
        Some(score)
    }

    /// Adds the specified number of lines at the bottom of the grid. The lines will be filled with blocks except for one column.
    pub fn add_garbage(&mut self, completed_lines: u64) {
        if completed_lines < 2 {
            return;
        }
        println!(
            "the garbage creating function was called for {} lines",
            completed_lines
        );
        let lines_to_add = if completed_lines == 4 {
            4
        } else {
            completed_lines - 1
        };

        /*****************************
         *     CHANGING THE GRID     *
         *****************************/

        // store the column index that will be empty
        let mut rng = rand::thread_rng();
        let empty = rng.gen::<u32>() % self.nb_columns;

        for _ in 0..lines_to_add {
            // move the rows and line_sum one line up
            self.line_sum
                .insert(self.nb_rows as usize, (self.nb_columns - 1) as u8);
            self.line_sum.remove(0);

            self.rows
                .insert(self.nb_rows as usize, vec![None; self.nb_columns as usize]);
            for x in 0..self.nb_columns {
                // add blocks in the entire line except in one column
                if x != empty {
                    self.rows[self.nb_rows as usize][x as usize] =
                        Some(Block::new(TetrisColor::Grey, x as i8, self.nb_rows as i8));
                } else {
                    self.rows[self.nb_rows as usize][x as usize] = None;
                }
            }
            self.rows.remove(0);

            // set the coordinates of the blocks higher
            for y in 0..self.nb_rows {
                for block in self.rows[y as usize].iter_mut().flatten() {
                    block.go_up(); // TODO add a function to move blocks a certain number of cells up
                }
            }
        }
    }

    /// Empty the grid.
    pub fn null(&mut self) {
        for row in self.rows.iter_mut() {
            for cell in row.iter_mut() {
                *cell = None;
            }
        }
        self.line_sum = vec![0; self.nb_rows as usize];
    }
}
