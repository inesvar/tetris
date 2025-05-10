//! Define the back-end of the tetris game.
#![doc = mermaid!("back_end/flowgraph.mmd")]
mod moving_primitives;
pub(in crate::app::player) mod render;
mod rotation_translation;
mod spatial_primitives;
mod tetris_grid;
mod tetromino;
mod tetromino_kind;

use self::{
    moving_primitives::ApplyRotationTranslation,
    rotation_translation::{Direction, RotationTranslation, RotationType},
    spatial_primitives::{Block, Position},
};
use crate::assets::TetrisColor;
use rand::seq::SliceRandom;
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use simple_mermaid::mermaid;

pub(in crate::app::player) use render::Render;

/// Tetromino piece among the 7 kinds in the game positioned on the grid.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Tetromino {
    kind: TetrominoKind,
    center: Position,
    blocks: [Block; 4],
    direction: Direction,
    pub(super) is_ghost: bool,
}

/// TetrominoKind describes the 7 types of Tetromino.
#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TetrominoKind {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

type GridLine = Vec<Option<TetrisColor>>;

/// Tetris grid containing blocks. It actually ontly contains their color as the coordinates of the blocks are given by their index in the matrix.
#[derive(Serialize, Deserialize)]
pub(in crate::app) struct TetrisGrid {
    nb_columns: u32,
    nb_rows: u32,
    matrix: Vec<GridLine>,
    line_sum: Vec<u32>,
    pub total_width: f64,
    pub total_height: f64,
    pub visible_width: f64,
    pub visible_height: f64,
}

/// Returns a random bag of TetrominoKind of the specified size using the given rng.
pub fn new_tetromino_bag(mut size_of_bag: u32, rng: &mut Pcg32) -> Vec<TetrominoKind> {
    if size_of_bag == 0 {
        size_of_bag = 1;
    }
    let mut tetromino_bag = vec![];
    let mut list = vec![];
    for _ in 0..(size_of_bag / 7) {
        for i in 0..7 {
            list.push(i);
        }
    }
    if size_of_bag % 7 != 0 {
        for i in 0..7 {
            list.push(i);
        }
    }
    // the list now has k elements, where k is the lower multiple of 7 higher or equal to size_of_bag
    list.shuffle(rng);

    for i in 0..size_of_bag {
        // only the first size_of_bag elements are used
        match list[i as usize] {
            0 => tetromino_bag.push(TetrominoKind::I),
            1 => tetromino_bag.push(TetrominoKind::O),
            2 => tetromino_bag.push(TetrominoKind::T),
            3 => tetromino_bag.push(TetrominoKind::S),
            4 => tetromino_bag.push(TetrominoKind::Z),
            5 => tetromino_bag.push(TetrominoKind::J),
            _ => tetromino_bag.push(TetrominoKind::L),
        }
    }
    tetromino_bag
}
