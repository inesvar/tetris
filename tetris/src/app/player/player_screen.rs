//! Implement [PlayerScreen].
use crate::settings::NB_NEXT_TETROMINO;
use rand_pcg::Pcg32;
use serde::Deserialize;
use std::cell::RefCell;
use tetris_core::{CircularArray, TetrisGrid, Tetromino, TetrominoGenerator};

/// Player screen contains all the elements that will appear on the screen relative to one player.
#[derive(Deserialize)]
pub struct PlayerScreen {
    /// Tetris grid.
    pub(in crate::app) grid: TetrisGrid,
    /// Number of lines cleared.
    pub(in crate::app) score: u64,
    /// Is set and reset during the update resp. when lines are cleared and when data is sent to the remote players
    pub(in crate::app) new_completed_lines: u64,
    /// The falling tetromino.
    pub(in crate::app) active_tetromino: Tetromino,
    /// The held tetromino piece rendered in the corner.
    pub(in crate::app) saved_tetromino: Option<Tetromino>,
    /// Next tetromino pieces rendered on the side.
    pub(in crate::app) fifo_next_tetromino: CircularArray<NB_NEXT_TETROMINO, Tetromino>,
    /// The shade of the active tetromino after hard drop.
    pub(in crate::app) ghost_tetromino: Tetromino,
    /// Flag not to be modified except in Serialize. Set to true.
    pub(in crate::app) serialize_as_msg: RefCell<bool>,
}

impl PlayerScreen {
    pub fn new(rng: &mut Pcg32, tetromino_bag: &mut TetrominoGenerator) -> Self {
        let grid = TetrisGrid::default();
        let active_tetromino = tetromino_bag.get(rng);
        let ghost_tetromino = active_tetromino.clone();
        let next_tetrominos = tetromino_bag.get_chunk::<NB_NEXT_TETROMINO>(rng);
        let fifo_next_tetromino = CircularArray::new(next_tetrominos);

        PlayerScreen {
            grid,
            score: 0,
            new_completed_lines: 0,
            active_tetromino,
            saved_tetromino: None,
            fifo_next_tetromino,
            ghost_tetromino,
            serialize_as_msg: true.into(),
        }
    }
}
