//! Re-export `struct`s [TetrisPlayer], [Tetromino], [TetrisGrid], [CircularBuffer] and [Position] as well as `enum`s [GameOverError], [TetrisResult], [TetrisColor], [BagType] and [TetrominoMove].
#![doc = simple_mermaid::mermaid!("tetris_core.mmd")]

mod circular_buffer;
mod tetris_grid;
mod tetris_player;
mod tetromino;
mod tetromino_generator;

pub(crate) use tetromino::TetrominoKind;
pub(crate) use tetromino_generator::TetrominoGenerator;

pub use circular_buffer::CircularBuffer;
pub use tetris_grid::{
    GameOverError, TetrisColor, TetrisGrid, TetrisResult, NB_VISIBLE_BUFFER_ROWS,
};
pub use tetris_player::TetrisPlayer;
pub use tetromino::{Position, Tetromino, TetrominoMove};
pub use tetromino_generator::BagType;
