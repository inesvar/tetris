//! Define the [render()](PlayerScreen::render()) and [constructor](PlayerScreen::empty()) of [PlayerScreen].
use super::core::{TetrisGrid, Tetromino};
use super::{CircularBuffer, PlayerScreen};
use crate::settings::{NB_COLUMNS, NB_HIDDEN_ROWS, NB_NEXT_TETROMINO, NB_ROWS};

impl PlayerScreen {
    pub fn empty() -> Self {
        PlayerScreen {
            grid: TetrisGrid::new(NB_COLUMNS, NB_ROWS, NB_HIDDEN_ROWS), //FIXME: this will not always be the case
            score: 0,
            game_over: false,
            new_completed_lines: 0,
            active_tetromino: Tetromino::default(),
            saved_tetromino: None,
            fifo_next_tetromino: CircularBuffer::<NB_NEXT_TETROMINO, Tetromino>::new(),
            ghost_tetromino: None,
            serialize_as_msg: true.into(),
        }
    }
}
