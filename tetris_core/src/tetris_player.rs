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

/// State of a tetris player.
///
/// According to the **Tetris Guideline**, the game elements include :
/// - **Matrix**: "the area where game play occurs."
/// - **Tetromino in Play**: "the player can manipulate this tetrimino by moving it right or left,
///   rotating it clockwise or counterclockwise, and Hard or Soft dropping it."
///   
///   NB: this implementation also supports 180° rotation.
/// - **Next Queue**: "allows the player to see the next tetrimino
///   that will be generated and put into play."
/// - **Hold Queue**: "allows the player to “hold” a falling tetrimino for as long as they wish."
///
/// [TetrisPlayer] is serializable and can be sent through the network
/// to implement multi-player tetris games, however this is not very efficient because
/// except the [TetrisPlayer::tetromino_in_play], most elements don't change between frames.
///
#[derive(Deserialize)]
pub struct TetrisPlayer {
    /// Tetris grid, or "matrix".
    pub matrix: TetrisGrid,
    /// Currently falling tetromino.
    pub tetromino_in_play: Tetromino,
    /// Tetrominos in the **Next Queue**.
    pub next_queue: CircularBuffer<Tetromino>,
    /// Tetromino in the **Hold Queue**, if there's one.
    pub hold_queue: Option<Tetromino>,
    /// Tetromino bag (used to refill the **Next Queue**).
    pub tetromino_bag: TetrominoGenerator,
    /// Total number of lines cleared.
    pub score: u64,
    // TODO : should be private
    pub new_completed_lines: u64,
    /// garbage_to_be_added is set before the update and reset during the update.
    garbage_to_be_added: u64,
    /// Flag not to be modified except in Serialize. Set to true.
    serialize_as_msg: RefCell<bool>,
}

/// All possible commands received by [TetrisPlayer].
pub enum TetrisCommand {
    /// A user command to move the [TetrisPlayer::tetromino_in_play]
    /// (there are 7 possible moves described by [TetrominoMove]).
    ///
    /// NB: [TetrominoMove::HardDrop] is automatically followed by [TetrisCommand::LockDown].
    Move(TetrominoMove),
    /// A user command to put the [TetrisPlayer::tetromino_in_play] in the **Hold Queue**
    /// (a new [TetrisPlayer::tetromino_in_play] will automatically be moved to its starting position).
    Hold,
    /// An in-game command to move the [TetrisPlayer::tetromino_in_play] down one block.
    Fall,
    /// An in-game command to **Lock Down** the tetromino.
    ///
    /// This command happens automatically:
    /// - immediately after [TetrominoMove::HardDrop];
    /// - a short time after [TetrisCommand::Fall] failed to move [TetrisPlayer::tetromino_in_play].
    ///
    /// This command consists in 3 steps :
    /// - add the [TetrisPlayer::tetromino_in_play] to the [TetrisPlayer::matrix] (this can fail with [GameOverError::LockOut]);
    /// - add the accumulated garbage to the [TetrisPlayer::matrix] (this can fail with [GameOverError::TopOut]);
    /// - move the new [TetrisPlayer::tetromino_in_play] to its starting position (this can fail with [GameOverError::BlockOut]).
    LockDown,
}

impl From<TetrominoMove> for TetrisCommand {
    fn from(value: TetrominoMove) -> Self {
        TetrisCommand::Move(value)
    }
}

impl TetrisPlayer {
    /// Creates a [TetrisPlayer], using `rng` to generate the first tetrominos.
    pub fn new(rng: &mut Pcg32) -> Self {
        let grid = TetrisGrid::default();
        let mut tetromino_bag = TetrominoGenerator::default();
        let active_tetromino = tetromino_bag.get(rng);
        let next_tetrominos = tetromino_bag.get_chunk(rng, NEXT_QUEUE_MAX_SIZE);
        let fifo_next_tetromino = CircularBuffer::new(next_tetrominos);

        TetrisPlayer {
            matrix: grid,
            score: 0,
            new_completed_lines: 0,
            tetromino_in_play: active_tetromino,
            hold_queue: None,
            next_queue: fifo_next_tetromino,
            tetromino_bag,
            garbage_to_be_added: 0,
            serialize_as_msg: true.into(),
        }
    }

    /// Replace the active tetromino by a tetromino from the next queue and return
    /// the previously active tetromino.
    fn replace_active_tetromino(&mut self, rng: &mut Pcg32) -> Tetromino {
        let mut swap = self.tetromino_bag.get(rng);
        self.next_queue.get_front_push_back(&mut swap);
        std::mem::swap(&mut self.tetromino_in_play, &mut swap);

        swap
    }

    /// Replace the active tetromino by the saved tetromino if it exists (or by a tetromino
    /// from the next queue) and return the previously active tetromino.
    fn replace_active_tetromino_using_stash(&mut self, rng: &mut Pcg32) -> Tetromino {
        if let Some(mut swap) = self.hold_queue.take() {
            std::mem::swap(&mut swap, &mut self.tetromino_in_play);
            swap
        } else {
            self.replace_active_tetromino(rng)
        }
    }

    fn stash(&mut self, rng: &mut Pcg32) -> TetrisResult {
        let mut previously_active = self.replace_active_tetromino_using_stash(rng);
        previously_active.reset();
        self.hold_queue = Some(previously_active);
        self.tetromino_in_play.try_enter_grid(&self.matrix)
    }

    fn record_new_completed_lines(&mut self, new_completed_lines: u64) {
        self.new_completed_lines += new_completed_lines;
        self.score += new_completed_lines;
    }

    fn lock_down(&mut self, rng: &mut Pcg32) -> TetrisResult {
        let previously_active = self.replace_active_tetromino(rng);

        let new_completed_lines = previously_active.lock_down(&mut self.matrix)?;
        self.record_new_completed_lines(new_completed_lines);

        self.matrix
            .apply_received_garbage(self.garbage_to_be_added)?;
        self.garbage_to_be_added = 0;

        self.tetromino_in_play.try_enter_grid(&self.matrix)
    }

    /// Adds `nb_completed_lines` to the number of garbage lines to be added later (during **Lock Down**).
    pub fn push_garbage(&mut self, nb_completed_lines: u64) {
        self.garbage_to_be_added += nb_completed_lines;
    }

    /// Returns the number of lines completed since the last call.
    pub fn get_lines_completed(&mut self) -> u64 {
        let lines = self.new_completed_lines;
        self.new_completed_lines = 0;
        lines
    }

    /// Empties the **Matrix** and puts a tetromino in its starting position.
    pub fn start(&mut self) {
        self.matrix.reset();
        let _ = self.tetromino_in_play.try_enter_grid(&self.matrix);
    }

    /// Tries to apply [TetrisCommand], returns [GameOverError] if the situation is a losing one,
    /// otherwise returns whether the [TetrisPlayer::tetromino_in_play] was moved or not.
    ///
    /// Refer to [TetrisCommand] documentation for more detail.
    pub fn try_apply(
        &mut self,
        order: TetrisCommand,
        rng: &mut Pcg32,
    ) -> Result<bool, GameOverError> {
        match order {
            TetrisCommand::Move(TetrominoMove::HardDrop) => {
                self.tetromino_in_play
                    .try_apply(TetrominoMove::HardDrop, &self.matrix);
                self.lock_down(rng).map(|_| true)
            }
            TetrisCommand::Move(tetromino_move) => Ok(self
                .tetromino_in_play
                .try_apply(tetromino_move, &self.matrix)),
            TetrisCommand::Fall => Ok(self
                .tetromino_in_play
                .try_apply(TetrominoMove::Fall, &self.matrix)),
            TetrisCommand::Hold => self.stash(rng).map(|_| true),
            TetrisCommand::LockDown => self.lock_down(rng).map(|_| true),
        }
    }

    /// Returns a hard-dropped copy of the [TetrisPlayer::tetromino_in_play].
    pub fn get_ghost_tetromino(&self) -> Tetromino {
        let mut ghost_tetromino = self.tetromino_in_play.clone();
        ghost_tetromino.try_apply(TetrominoMove::HardDrop, &self.matrix);
        ghost_tetromino
    }
}
