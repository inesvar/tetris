//! Re-export `struct`s [Tetromino], [TetrisGrid], [TetrominoGenerator], [CircularArray] and [Position] as well as `enum`s [GameOverError], [TetrisResult], [TetrisColor], [BagType] and [TetrominoMove].
#![doc = simple_mermaid::mermaid!("tetris_core.mmd")]

mod circular_array;
mod tetris_grid;
mod tetromino;
mod tetromino_generator;

pub(crate) use tetromino::TetrominoKind;

pub use circular_array::CircularArray;
pub use tetris_grid::{
    GameOverError, TetrisColor, TetrisGrid, TetrisResult, NB_VISIBLE_BUFFER_ROWS,
};
pub use tetromino::{Position, Tetromino, TetrominoMove};
pub use tetromino_generator::{BagType, TetrominoGenerator};
