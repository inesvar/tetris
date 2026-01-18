//! Re-export `struct`s [Tetromino], [TetrisGrid], [TetrominoGenerator] and [Position] as well as `enum`s [GameOverError], [TetrominoKind] and [TetrominoMove].
#![doc = simple_mermaid::mermaid!("core/core.mmd")]

mod tetris_grid;
mod tetromino;
mod tetromino_generator;
mod tetromino_move;

pub(in crate::app::player) use tetris_grid::NB_VISIBLE_BUFFER_ROWS;
pub(in crate::app) use tetris_grid::{GameOverError, TetrisGrid};
pub(super) use tetromino::Position;
pub use tetromino::TetrisColor;
pub(in crate::app) use tetromino::Tetromino;
pub(in crate::app::player) use tetromino::TetrominoKind;
pub(in crate::app::player) use tetromino_generator::TetrominoGenerator;
pub(in crate::app::player) use tetromino_move::TetrominoMove;
