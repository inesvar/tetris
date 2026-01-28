//! Re-export `struct`s [PlayerScreen], [Tetromino], [TetrisGrid], [TetrominoGenerator], [CircularBuffer] and [Position] as well as `enum`s [GameOverError], [TetrisResult], [TetrisColor], [BagType] and [TetrominoMove].
#![doc = simple_mermaid::mermaid!("tetris_core.mmd")]

mod circular_buffer;
mod player_screen;
mod tetris_grid;
mod tetromino;
mod tetromino_generator;

pub(crate) use tetromino::TetrominoKind;

pub use circular_buffer::CircularBuffer;
pub use player_screen::PlayerScreen;
pub use tetris_grid::{
    GameOverError, TetrisColor, TetrisGrid, TetrisResult, NB_VISIBLE_BUFFER_ROWS,
};
pub use tetromino::{Position, Tetromino, TetrominoMove};
pub use tetromino_generator::{BagType, TetrominoGenerator};
