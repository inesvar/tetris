//! Implement [TetrisPlayer].
use super::{
    CircularBuffer, GameOverError, TetrisGrid, TetrisResult, Tetromino, TetrominoGenerator,
    TetrominoMove,
};
use rand_pcg::Pcg32;
use serde::Deserialize;
use std::cell::RefCell;

mod custom_serialize_as_msg;

pub const NEXT_QUEUE_MAX_SIZE: usize = 6;

/// Player screen contains all the elements that will appear on the screen relative to one player.
#[derive(Deserialize)]
pub struct TetrisPlayer {
    /// Tetris grid.
    pub grid: TetrisGrid,
    /// Number of lines cleared.
    pub score: u64,
    /// Is set and reset during the update resp. when lines are cleared and when data is sent to the remote players
    pub new_completed_lines: u64,
    /// The falling tetromino.
    pub active_tetromino: Tetromino,
    /// The held tetromino piece rendered in the corner.
    pub saved_tetromino: Option<Tetromino>,
    /// Next tetromino pieces rendered on the side.
    pub fifo_next_tetromino: CircularBuffer<Tetromino>,
    /// The shade of the active tetromino after hard drop.
    pub ghost_tetromino: Tetromino,
    pub tetromino_bag: TetrominoGenerator,
    /// garbage_to_be_added is set before the update and reset during the update.
    pub garbage_to_be_added: u64,
    /// Flag not to be modified except in Serialize. Set to true.
    pub serialize_as_msg: RefCell<bool>,
}

pub enum TetrisOrder {
    PlayerMovesTetromino(TetrominoMove),
    PlayerStashesTetromino,
    TetrominoFalls,
    TetrominoLocksDown,
}

impl From<TetrominoMove> for TetrisOrder {
    fn from(value: TetrominoMove) -> Self {
        TetrisOrder::PlayerMovesTetromino(value)
    }
}

impl TetrisPlayer {
    pub fn new(rng: &mut Pcg32) -> Self {
        let grid = TetrisGrid::default();
        let mut tetromino_bag = TetrominoGenerator::default();
        let active_tetromino = tetromino_bag.get(rng);
        let ghost_tetromino = active_tetromino.clone();
        let next_tetrominos = tetromino_bag.get_chunk(rng, NEXT_QUEUE_MAX_SIZE);
        let fifo_next_tetromino = CircularBuffer::new(next_tetrominos);

        TetrisPlayer {
            grid,
            score: 0,
            new_completed_lines: 0,
            active_tetromino,
            saved_tetromino: None,
            fifo_next_tetromino,
            ghost_tetromino,
            tetromino_bag,
            garbage_to_be_added: 0,
            serialize_as_msg: true.into(),
        }
    }

    /// Replace the active tetromino by a tetromino from the next queue and return
    /// the previously active tetromino.
    fn replace_active_tetromino(&mut self, rng: &mut Pcg32) -> Tetromino {
        let mut swap = self.tetromino_bag.get(rng);
        self.fifo_next_tetromino.get_front_push_back(&mut swap);
        std::mem::swap(&mut self.active_tetromino, &mut swap);

        swap
    }

    /// Replace the active tetromino by the saved tetromino if it exists (or by a tetromino
    /// from the next queue) and return the previously active tetromino.
    fn replace_active_tetromino_using_stash(&mut self, rng: &mut Pcg32) -> Tetromino {
        if let Some(mut swap) = self.saved_tetromino.take() {
            std::mem::swap(&mut swap, &mut self.active_tetromino);
            swap
        } else {
            self.replace_active_tetromino(rng)
        }
    }

    fn stash(&mut self, rng: &mut Pcg32) -> TetrisResult {
        let mut previously_active = self.replace_active_tetromino_using_stash(rng);
        previously_active.reset();
        self.saved_tetromino = Some(previously_active);
        self.active_tetromino.try_enter_grid(&self.grid)
    }

    fn record_new_completed_lines(&mut self, new_completed_lines: u64) {
        self.new_completed_lines += new_completed_lines;
        self.score += new_completed_lines;
    }

    fn lock_down(&mut self, rng: &mut Pcg32) -> TetrisResult {
        let previously_active = self.replace_active_tetromino(rng);

        let new_completed_lines = previously_active.lock_down(&mut self.grid)?;
        self.record_new_completed_lines(new_completed_lines);

        self.grid.apply_received_garbage(self.garbage_to_be_added)?;
        self.garbage_to_be_added = 0;

        self.active_tetromino.try_enter_grid(&self.grid)
    }

    pub fn push_garbage(&mut self, completed_lines: u64) {
        self.garbage_to_be_added += completed_lines;
    }

    pub fn get_lines_completed(&mut self) -> u64 {
        let lines = self.new_completed_lines;
        self.new_completed_lines = 0;
        lines
    }

    pub fn start(&mut self) {
        self.grid.reset();
        let _ = self.active_tetromino.try_enter_grid(&self.grid);
    }

    pub fn try_apply(
        &mut self,
        order: TetrisOrder,
        rng: &mut Pcg32,
    ) -> Result<bool, GameOverError> {
        match order {
            TetrisOrder::PlayerMovesTetromino(tetromino_move) => {
                Ok(self.active_tetromino.try_apply(tetromino_move, &self.grid))
            }
            TetrisOrder::TetrominoFalls => Ok(self
                .active_tetromino
                .try_apply(TetrominoMove::Fall, &self.grid)),
            TetrisOrder::PlayerStashesTetromino => self.stash(rng).map(|_| true),
            TetrisOrder::TetrominoLocksDown => self.lock_down(rng).map(|_| true),
        }
    }
}
