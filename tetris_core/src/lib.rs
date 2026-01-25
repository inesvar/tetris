//! Re-export `struct`s [Tetromino], [TetrisGrid], [TetrominoGenerator] and [Position] as well as `enum`s [GameOverError], [TetrisColor], [BagType] and [TetrominoMove].
#![doc = simple_mermaid::mermaid!("tetris_core.mmd")]

mod tetris_grid;
mod tetromino;
mod tetromino_generator;

pub(crate) use tetromino::TetrominoKind;

pub use tetris_grid::{GameOverError, TetrisColor, TetrisGrid, NB_VISIBLE_BUFFER_ROWS};
pub use tetromino::{Position, Tetromino, TetrominoMove};
pub use tetromino_generator::{BagType, TetrominoGenerator};
