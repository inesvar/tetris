//! Re-export `struct`s [Tetromino], [TetrisGrid], [TetrominoGenerator] and [Position] as well as `enum`s [GameOverError], [TetrominoKind] and [TetrominoMove].
#![doc = mermaid!("core/core.mmd")]

mod tetris_grid;
mod tetromino;
mod tetromino_generator;
mod tetromino_move;

use crate::assets::TetrisColor;
use simple_mermaid::mermaid;

pub(in crate::app) use tetris_grid::{GameOverError, TetrisGrid};
pub(super) use tetromino::Position;
pub(in crate::app) use tetromino::Tetromino;
pub(in crate::app::player) use tetromino::TetrominoKind;
pub(in crate::app::player) use tetromino_generator::TetrominoGenerator;
pub(in crate::app::player) use tetromino_move::TetrominoMove;
