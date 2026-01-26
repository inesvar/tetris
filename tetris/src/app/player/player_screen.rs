//! Implement [PlayerScreen].
use super::{CircularArray, PlayerScreen};
use crate::settings::NB_NEXT_TETROMINO;
use rand_pcg::Pcg32;
use tetris_core::{TetrisGrid, TetrominoGenerator};

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
