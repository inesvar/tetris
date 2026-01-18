//! Re-export `struct`s [Tetromino], [TetrisGrid], [TetrominoGenerator] and [Position] as well as `enum`s [GameOverError], [TetrominoKind] and [TetrominoMove].
#![doc = simple_mermaid::mermaid!("core.mmd")]

mod tetris_grid;
mod tetromino;
mod tetromino_generator;
mod tetromino_move;

pub use tetris_grid::NB_VISIBLE_BUFFER_ROWS;
pub use tetris_grid::{GameOverError, TetrisGrid};
pub use tetromino::Position;
pub use tetromino::TetrisColor;
pub use tetromino::Tetromino;
pub use tetromino::TetrominoKind;
pub use tetromino_generator::TetrominoGenerator;
pub use tetromino_move::TetrominoMove;

#[macro_export]
macro_rules! concatln {
    ( $( $line:expr ),* $(,)? ) => {
        concat!( $( $line, "\n", )*)
    };
}
