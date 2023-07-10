
use crate::settings::BLOCK_SIZE;
use graphics::types::Matrix2d;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TetrisGrid {
    pub x: f64,
    pub y: f64,
    pub nb_columns: u32,
    pub nb_rows: u32,
    pub line_sum: Vec<u8>,
    pub total_width: f64,
    pub total_height: f64,
    pub visible_width: f64,
    pub visible_height: f64,
    pub transform: Matrix2d<f64>,
}

impl TetrisGrid {
    pub fn new(x: f64, y: f64, nb_columns: u32, nb_rows: u32) -> TetrisGrid {

        let line_sum = vec![0; nb_rows as usize];
        TetrisGrid {
            x,
            y,
            nb_columns,
            nb_rows,
            line_sum,
            total_width: nb_columns as f64 * BLOCK_SIZE,
            total_height: nb_rows as f64 * BLOCK_SIZE,
            visible_width: nb_columns as f64 * BLOCK_SIZE,
            visible_height: (nb_rows - 2) as f64 * BLOCK_SIZE,
            transform: Matrix2d::default(),
        }
    }
}
