//! Define `struct` [Tetromino], `struct` [TetrisGrid], `enum` [TetrominoKind] and `trait` [UseTetromino]
#![doc = mermaid!("back_end/back_end_flowgraph.mmd")]
mod moving_primitives;
pub(in crate::app::player) mod render;
mod rotation_translation;
mod spatial_primitives;
mod tetris_grid;
mod tetromino;
mod tetromino_kind;

use self::{
    moving_primitives::ApplyRotationTranslation,
    rotation_translation::{RotationTranslation, RotationType},
    spatial_primitives::{Block, Direction, Position},
};
use crate::assets::TetrisColor;
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use simple_mermaid::mermaid;

pub(in crate::app::player) use render::Render;

/// Tetromino.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub(crate) struct Tetromino {
    kind: TetrominoKind,
    center: Position,
    blocks: [Block; 4],
    direction: Direction,
    pub(super) is_ghost: bool,
}

/// Seven types of Tetromino.
#[derive(PartialEq, Copy, Clone, Serialize, Deserialize, Debug)]
pub(in crate::app::player) enum TetrominoKind {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

type GridLine = Vec<Option<TetrisColor>>;

/// Tetris grid.
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

/// Move or create a [Tetromino] on a [TetrisGrid].
pub(in crate::app::player) trait UseTetromino: Sized {
    /// Return whether the tetromino could be moved one cell down.
    fn fall(&mut self, grid: &TetrisGrid) -> Result<(), ()>;

    /// Move the tetromino down until it's not possible anymore.
    fn hard_drop(&mut self, grid: &TetrisGrid);

    /// Move the tetromino one cell to the left if it's possible.
    fn left(&mut self, grid: &TetrisGrid);

    /// Move the tetromino one cell to the right if it's possible.
    fn right(&mut self, grid: &TetrisGrid);

    /// Turn the tetromino clockwise if it's possible, eventually using wall-kicks.
    fn turn_clockwise(&mut self, grid: &TetrisGrid);

    /// Turn the tetromino counterclockwise if it's possible, eventually using wall-kicks.
    fn turn_counterclockwise(&mut self, grid: &TetrisGrid);

    // Return an Option eventually containing a Tetromino if its starting position is empty.
    fn new(kind: TetrominoKind, grid: &TetrisGrid) -> Option<Self>;

    // Return a Tetromino at its starting position without checking that this place is empty.
    fn new_unchecked(kind: TetrominoKind) -> Self;

    // TODO can't this cause a collision??
    // TODO the direction is not reset ??? => test this but using new_unchecked seems better
    /// Reset the Tetromino at its starting position.
    fn reset_position(&mut self);

    // TODO so it's hard dropped separately? it could be hard droped here !
    /// Return a ghost copy of the Tetromino.
    fn make_ghost_copy(&mut self) -> Self;

    /// Return a random bag of [TetrominoKind] of the specified size using the given rng.
    fn new_tetromino_bag(size_of_bag: u32, rng: &mut Pcg32) -> Vec<TetrominoKind>;
}
