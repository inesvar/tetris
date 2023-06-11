use crate::assets::TetrisColor;
use crate::block::Block;
use crate::point::Transformable;
use crate::settings::{BLOCK_SIZE};
use crate::tetromino::Tetromino;
use graphics::types::Matrix2d;
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TetrisGrid {
    pub x: f64,
    pub y: f64,
    pub nb_columns: u32,
    pub nb_rows: u32,
    pub rows: Vec<Vec<Option<Block>>>,
    pub line_sum: Vec<u8>,
    pub total_width: f64,
    pub total_height: f64,
    pub visible_width: f64,
    pub visible_height: f64,
    pub transform: Matrix2d<f64>,
}

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

    pub fn freeze_tetromino(&mut self, tetromino: &mut Tetromino) -> u64 {
        let mut blocks = tetromino.split();
        for block in &mut blocks {
            self.rows[block.position.y as usize][block.position.x as usize] = Some(*block);
            self.line_sum[block.position.y as usize] += 1;
        }
        let mut score = 0;
        for y in 0..self.nb_rows {
            if self.line_sum[y as usize] == self.nb_columns as u8 {
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
        score
    }

    pub fn add_garbage(&mut self, completed_lines: u64) -> Result<(),()> {
        println!("the garbage creating function was called");
        if completed_lines < 2 {
            Ok(())
        } else if self.line_sum[(completed_lines - 2) as usize] > 0 {

            let mut rng = rand::thread_rng();
            let empty = rng.gen::<u32>()%self.nb_columns;

            for _ in 0..=(completed_lines - 2) {
                self.line_sum.insert(self.nb_rows as usize, (self.nb_columns - 1) as u8);
                self.line_sum.remove(0);
                 
                self.rows.insert(self.nb_rows as usize, vec![None; self.nb_columns as usize]);
                for x in 0..self.nb_columns {
                    if x != empty {
                        self.rows[self.nb_rows as usize][x as usize] = Some(Block::new(TetrisColor::Grey, x as i8, self.nb_rows as i8));
                    } else {
                        self.rows[self.nb_rows as usize][x as usize] = None;
                    }
                }
                self.rows.remove(0);
                // move all block up
                for y in 0..self.nb_rows {
                    for block in self.rows[y as usize].iter_mut().flatten() {
                        block.go_up();
                    }
                }
            }
            println!("{} lines were removed", completed_lines - 2);
            Err(())
        } else {

            let mut rng = rand::thread_rng();
            let empty = rng.gen::<u32>()%self.nb_columns;

            for _ in 0..=(completed_lines - 2) {
                self.line_sum.insert(self.nb_rows as usize, (self.nb_columns - 1) as u8);
                self.line_sum.remove(0);
                 
                self.rows.insert(self.nb_rows as usize, vec![None; self.nb_columns as usize]);
                for x in 0..self.nb_columns {
                    if x != empty {
                        self.rows[self.nb_rows as usize][x as usize] = Some(Block::new(TetrisColor::Grey, x as i8, self.nb_rows as i8));
                    } else {
                        self.rows[self.nb_rows as usize][x as usize] = None;
                    }
                }
                self.rows.remove(0);
                // move all block up
                for y in 0..self.nb_rows {
                    for block in self.rows[y as usize].iter_mut().flatten() {
                        block.go_up();
                    }
                }
            }
            println!("{} lines were removed", completed_lines - 2);
            Ok(())
        }
    }

    pub fn null(&mut self) {
        for row in self.rows.iter_mut() {
            for cell in row.iter_mut() {
                *cell = None;
            }
        }
        self.line_sum = vec![0; self.nb_rows as usize];
    }
}
