//! Re-export `struct`s [Tetromino], [TetrisGrid], [TetrominoKind] as well as `enum` [GameOverError].
#![doc = mermaid!("core/tetromino.mmd")]
mod moving_primitives;
mod rotation_translation;
mod spatial_primitives;
mod tetris_grid;
mod tetromino;
mod tetromino_kind;
mod tetromino_move;

use crate::assets::TetrisColor;
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use simple_mermaid::mermaid;

pub(super) use spatial_primitives::Position;
pub(in crate::app) use tetris_grid::{GameOverError, TetrisGrid};
pub(crate) use tetromino::Tetromino;
pub(in crate::app::player) use tetromino_kind::TetrominoKind;
pub(in crate::app::player) use tetromino_move::TetrominoMove;
