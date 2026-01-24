//! Define the [constructor](PlayerScreen::empty()) of [PlayerScreen].
use tetris_core::{TetrisGrid, Tetromino};
use super::{CircularArray, PlayerScreen};
use crate::settings::NB_NEXT_TETROMINO;

impl PlayerScreen {
    pub fn empty() -> Self {
        PlayerScreen {
            grid: TetrisGrid::default(),
            score: 0,
            game_over: false,
            new_completed_lines: 0,
            active_tetromino: Tetromino::default(),
            saved_tetromino: None,
            fifo_next_tetromino: CircularArray::new([Tetromino::default(); NB_NEXT_TETROMINO]),
            ghost_tetromino: None,
            serialize_as_msg: true.into(),
        }
    }
}
