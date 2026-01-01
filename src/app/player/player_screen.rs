//! Define the [constructor](PlayerScreen::empty()) of [PlayerScreen].
use super::core::{TetrisGrid, Tetromino};
use super::{CircularBuffer, PlayerScreen};
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
            fifo_next_tetromino: CircularBuffer::new([Tetromino::default(); NB_NEXT_TETROMINO]),
            ghost_tetromino: None,
            serialize_as_msg: true.into(),
        }
    }
}
