//! Define `trait` [UseTetromino], re-export `trait` [Render], `struct` [Tetromino], `struct` [TetrisGrid] and `enum` [TetrominoKind].
#![doc = mermaid!("back_end/back_end_flowgraph.mmd")]
mod moving_primitives;
mod render;
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
pub(in crate::app) use tetris_grid::TetrisGrid;
pub(crate) use tetromino::Tetromino;
pub(in crate::app::player) use tetromino_kind::TetrominoKind;

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
