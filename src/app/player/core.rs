//! Re-export `struct`s [Tetromino], [TetrisGrid], [TetrominoKind] as well as `enum` [GameOverError].
mod tetris_grid;
mod tetromino;
mod tetromino_bag;
mod tetromino_move;

use crate::assets::TetrisColor;
use serde::{Deserialize, Serialize};
use simple_mermaid::mermaid;

pub(in crate::app) use tetris_grid::{GameOverError, TetrisGrid};
pub(super) use tetromino::Position;
pub(crate) use tetromino::Tetromino;
pub(in crate::app::player) use tetromino::TetrominoKind;
pub(in crate::app::player) use tetromino_bag::TetrominoBag;
pub(in crate::app::player) use tetromino_move::TetrominoMove;
