#![doc = simple_mermaid::mermaid!("tetris_core.mmd")]

mod circular_buffer;
mod tetris_grid;
mod tetris_player;
mod tetromino;
mod tetromino_generator;

pub(crate) use circular_buffer::CircularBuffer;
pub(crate) use tetromino::TetrominoKind;
pub(crate) use tetromino_generator::TetrominoGenerator;

// used to update the active tetromino
pub use tetris_grid::{GameOverError, TetrisResult};
pub use tetromino::TetrominoMove;
// used to render the TetrisPlayer
pub use tetris_grid::{TetrisColor, TetrisGrid, NB_VISIBLE_BUFFER_ROWS};
pub use tetris_player::TetrisPlayer;
pub use tetromino::{Position, Tetromino};
// currently unused, TODO: propose different constructors for TetrisPlayer
pub use tetromino_generator::BagType;
