//! Define `trait` [UseTetromino], re-export `struct`s [Tetromino], [TetrisGrid], [TetrominoKind] as well as `enum` [CoreError].
#![doc = mermaid!("core/core_flowgraph.mmd")]
mod moving_primitives;
mod rotation_translation;
mod spatial_primitives;
mod tetris_grid;
mod tetromino;
mod tetromino_kind;

use crate::assets::TetrisColor;
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use simple_mermaid::mermaid;

pub(super) use spatial_primitives::Position;
pub(in crate::app) use tetris_grid::{CoreError, TetrisGrid};
pub(crate) use tetromino::Tetromino;
pub(in crate::app::player) use tetromino_kind::TetrominoKind;

/// Use a [Tetromino] on a [TetrisGrid].
#[doc = mermaid!("core/use_tetromino_flowgraph.mmd")]
pub(in crate::app::player) trait UseTetromino: Sized {
    /// Return whether the tetromino could be moved one cell down.
    fn fall(&mut self, grid: &TetrisGrid) -> Result<(), ()>;

    /// Move the tetromino down until it's not possible anymore.
    fn hard_drop(&mut self, grid: &TetrisGrid);

    /// Move the tetromino one cell to the left if it's possible.
    fn left(&mut self, grid: &TetrisGrid);

    /// Move the tetromino one cell to the right if it's possible.
    fn right(&mut self, grid: &TetrisGrid);

    /// Turn the tetromino 180° if it's possible.
    fn turn_half_turn(&mut self, grid: &TetrisGrid);

    /// Turn the tetromino clockwise if it's possible, eventually using wall-kicks.
    fn turn_clockwise(&mut self, grid: &TetrisGrid);

    /// Turn the tetromino counterclockwise if it's possible, eventually using wall-kicks.
    fn turn_counterclockwise(&mut self, grid: &TetrisGrid);

    /// Return a new Tetromino at its starting position.
    fn new(kind: TetrominoKind) -> Self;

    /// Reset the Tetromino at its starting position.
    fn reset(&mut self);

    /// Check whether the Tetromino can enter the grid.
    fn can_enter_grid(&self, grid: &TetrisGrid) -> bool;

    /// Lock down in the grid.
    /// Return error on LockOut and number of completed lines on success.
    fn lock_down(self, grid: &mut TetrisGrid) -> Result<u64, CoreError>;

    /// Return a random bag of [TetrominoKind] of the specified size using the given rng.
    fn new_tetromino_bag(size_of_bag: u32, rng: &mut Pcg32) -> Vec<TetrominoKind>;
}
