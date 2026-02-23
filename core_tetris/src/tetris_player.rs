//! Implement [TetrisPlayer].
use super::{
    BagType, CircularBuffer, GameOverError, TetrisGrid, TetrisResult, Tetromino,
    TetrominoGenerator, TetrominoMove,
};
use rand::Rng;
use serde::Deserialize;
use std::{cell::RefCell, fmt::Display};

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
    /// Creates a [TetrisPlayer]:
    /// - using `rng` and `bag_type` to generate tetrominos;
    /// - using [TetrisGrid::DEFAULT_NB_COLUMNS], [TetrisGrid::DEFAULT_NB_MATRIX_ROWS] and [TetrisGrid::DEFAULT_NB_BUFFER_ROWS] to create the matrix.
    pub fn default<R: Rng>(rng: &mut R, bag_type: BagType) -> Self {
        TetrisPlayer::new(
            rng,
            bag_type,
            TetrisGrid::DEFAULT_NB_COLUMNS,
            TetrisGrid::DEFAULT_NB_MATRIX_ROWS,
            TetrisGrid::DEFAULT_NB_BUFFER_ROWS,
        )
    }

    /// Creates a [TetrisPlayer]:
    /// - using `rng` and `bag_type` to generate tetrominos;
    /// - using [TetrisGrid::COMPACT_NB_COLUMNS], [TetrisGrid::COMPACT_NB_MATRIX_ROWS] and [TetrisGrid::COMPACT_NB_BUFFER_ROWS] to create the matrix.
    pub fn compact<R: Rng>(rng: &mut R, bag_type: BagType) -> Self {
        TetrisPlayer::new(
            rng,
            bag_type,
            TetrisGrid::COMPACT_NB_COLUMNS,
            TetrisGrid::COMPACT_NB_MATRIX_ROWS,
            TetrisGrid::COMPACT_NB_BUFFER_ROWS,
        )
    }

    /// Creates a [TetrisPlayer]:
    /// - using `rng` and `bag_type` to generate tetrominos;
    /// - using `nb_columns`, `nb_matrix_rows` and `nb_buffer_rows` to create the matrix.
    ///
    /// # Panics
    ///
    /// If `nb_columns` or `nb_matrix_rows` or `nb_buffer_rows` aren't in the expected range (see [TetrisGrid::new]).
    pub fn new<R: Rng>(
        rng: &mut R,
        bag_type: BagType,
        nb_columns: u32,
        nb_matrix_rows: u32,
        nb_buffer_rows: u32,
    ) -> Self {
        let matrix = TetrisGrid::new(nb_columns, nb_matrix_rows, nb_buffer_rows);
        let mut tetromino_bag = TetrominoGenerator::new(bag_type);
        let mut tetromino_in_play = tetromino_bag.get(rng);
        let next_tetrominos = tetromino_bag.get_chunk(rng, NEXT_QUEUE_MAX_SIZE);
        let next_queue = CircularBuffer::new(next_tetrominos);
        tetromino_in_play.enter_grid(&matrix);

        TetrisPlayer {
            matrix,
            score: 0,
            new_completed_lines: 0,
            tetromino_in_play,
            hold_queue: None,
            next_queue,
            tetromino_bag,
            garbage_to_be_added: 0,
            serialize_as_msg: true.into(),
        }
    }

    pub fn bag_type(&self) -> BagType {
        self.tetromino_bag.bag_type()
    }

    fn swap_tetromino_in_play_with(&mut self, new: &mut Tetromino) {
        std::mem::swap(&mut self.tetromino_in_play, new);
        self.tetromino_in_play.enter_grid(&self.matrix);
    }

    /// Replace the tetromino in play by a tetromino from the **Next Queue** and return
    /// the previous tetromino in play.
    fn replace_tetromino_in_play<R: Rng>(&mut self, rng: &mut R) -> Tetromino {
        let mut swap = self.tetromino_bag.get(rng);
        self.next_queue.get_front_push_back(&mut swap);
        self.swap_tetromino_in_play_with(&mut swap);

        swap
    }

    /// Replace the tetromino in play by the tetromino in the **Hold Queue** if it exists
    /// (alternatively by a tetromino from the **Next Queue**) and return the previous tetromino in play.
    fn replace_tetromino_in_play_using_hold_queue<R: Rng>(&mut self, rng: &mut R) -> Tetromino {
        if let Some(mut swap) = self.hold_queue.take() {
            self.swap_tetromino_in_play_with(&mut swap);
            swap
        } else {
            self.replace_tetromino_in_play(rng)
        }
    }

    /// Put the tetromino in play in the **Hold Queue** and put the new tetromino in play
    /// in its starting position.
    fn hold_tetromino<R: Rng>(&mut self, rng: &mut R) -> TetrisResult {
        let mut previously_active = self.replace_tetromino_in_play_using_hold_queue(rng);
        previously_active.reset();
        self.hold_queue = Some(previously_active);
        self.tetromino_in_play.is_valid_in_grid(&self.matrix)
    }

    fn record_new_completed_lines(&mut self, new_completed_lines: u64) {
        self.new_completed_lines += new_completed_lines;
        self.score += new_completed_lines;
    }

    fn lock_down<R: Rng>(&mut self, rng: &mut R) -> TetrisResult {
        let previously_active = self.replace_tetromino_in_play(rng);

        let new_completed_lines = previously_active.lock_down(&mut self.matrix)?;
        self.record_new_completed_lines(new_completed_lines);

        self.matrix
            .apply_received_garbage(self.garbage_to_be_added)?;
        self.garbage_to_be_added = 0;

        self.tetromino_in_play.is_valid_in_grid(&self.matrix)
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
    pub fn reset(&mut self) {
        self.matrix.reset();
    }

    /// Tries to apply [TetrisCommand], returns [GameOverError] if the situation is a losing one,
    /// otherwise returns whether the [TetrisPlayer::tetromino_in_play] was moved or not.
    ///
    /// Refer to [TetrisCommand] documentation for more detail.
    pub fn try_apply<R: Rng>(
        &mut self,
        order: TetrisCommand,
        rng: &mut R,
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
            TetrisCommand::Hold => self.hold_tetromino(rng).map(|_| true),
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

impl Display for TetrisPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = self.matrix.clone();
        let tetromino = self.tetromino_in_play.clone();

        let _ = tetromino.lock_down(&mut grid);

        write!(f, "{}", grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockRng;
    use rstest::rstest;

    #[rstest]
    #[case(TetrisPlayer::compact(&mut MockRng::default(), BagType::NoBag), concat!(
            "---------\n",
            "    OO   \n",
            "    OO   \n",
            "---------\n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "         \n",
            "---------\n",
        ))]
    #[case(TetrisPlayer::new(&mut MockRng::default(), BagType::NoBag, TetrisGrid::DEFAULT_NB_COLUMNS, TetrisGrid::COMPACT_NB_MATRIX_ROWS, TetrisGrid::COMPACT_NB_BUFFER_ROWS), concat!(
            "----------\n",
            "    OO    \n",
            "    OO    \n",
            "----------\n",
            "          \n",
            "          \n",
            "          \n",
            "          \n",
            "          \n",
            "          \n",
            "----------\n",
        ))]
    fn display_is_correct(#[case] player: TetrisPlayer, #[case] expected: &str) {
        assert_eq!(
            player.to_string(),
            expected,
            "player:\n{}, expected:\n{}",
            player,
            expected
        );
    }

    #[rstest]
    #[case::compact(
        TetrisGrid::COMPACT_NB_COLUMNS,
        TetrisGrid::COMPACT_NB_MATRIX_ROWS,
        TetrisGrid::COMPACT_NB_BUFFER_ROWS
    )]
    #[case::default(
        TetrisGrid::DEFAULT_NB_COLUMNS,
        TetrisGrid::DEFAULT_NB_MATRIX_ROWS,
        TetrisGrid::DEFAULT_NB_BUFFER_ROWS
    )]
    #[case::minimal(4, 6, 2)]
    fn new_does_not_panic(
        #[values(&mut MockRng::default())] rng: &mut MockRng,
        #[values(BagType::NoBag, BagType::Bag7, BagType::Bag14)] bag_type: BagType,
        #[case] nb_columns: u32,
        #[case] nb_matrix_rows: u32,
        #[case] nb_buffer_rows: u32,
    ) {
        TetrisPlayer::new(rng, bag_type, nb_columns, nb_matrix_rows, nb_buffer_rows);
    }
}
