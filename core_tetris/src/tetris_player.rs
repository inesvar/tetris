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
    /// Tetris grid.
    pub grid: TetrisGrid,
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
    /// received_garbage_lines is set before the update and reset during the update.
    received_garbage_lines: u64,
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
            grid: matrix,
            score: 0,
            new_completed_lines: 0,
            tetromino_in_play,
            hold_queue: None,
            next_queue,
            tetromino_bag,
            received_garbage_lines: 0,
            serialize_as_msg: true.into(),
        }
    }

    pub fn bag_type(&self) -> BagType {
        self.tetromino_bag.bag_type()
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
                    .try_apply(TetrominoMove::HardDrop, &self.grid);
                self.lock_down(rng).map(|_| true)
            }
            TetrisCommand::Move(tetromino_move) => {
                Ok(self.tetromino_in_play.try_apply(tetromino_move, &self.grid))
            }
            TetrisCommand::Fall => Ok(self
                .tetromino_in_play
                .try_apply(TetrominoMove::Fall, &self.grid)),
            TetrisCommand::Hold => self.hold_tetromino(rng).map(|_| true),
            TetrisCommand::LockDown => self.lock_down(rng).map(|_| true),
        }
    }

    /// Swap `tetromino_in_play` and `swap`.
    fn swap_tetromino_in_play_with(&mut self, swap: &mut Tetromino) {
        std::mem::swap(&mut self.tetromino_in_play, swap);
        self.tetromino_in_play.enter_grid(&self.grid);
    }

    /// Get a new `tetromino_in_play` from the **Next Queue** and return the previous one.
    fn replace_tetromino_in_play<R: Rng>(&mut self, rng: &mut R) -> Tetromino {
        let mut swap = self.tetromino_bag.get(rng);
        self.next_queue.get_front_push_back(&mut swap);
        self.swap_tetromino_in_play_with(&mut swap);

        swap
    }

    /// Get a new `tetromino_in_play` from the **Hold Queue**  (alternatively from the **Next Queue**)
    /// and return the previous one.
    fn replace_tetromino_in_play_using_hold_queue<R: Rng>(&mut self, rng: &mut R) -> Tetromino {
        if let Some(mut swap) = self.hold_queue.take() {
            self.swap_tetromino_in_play_with(&mut swap);
            swap
        } else {
            self.replace_tetromino_in_play(rng)
        }
    }

    /// Put `tetromino` in the **Hold Queue**.
    fn put_in_hold_queue(&mut self, mut tetromino: Tetromino) {
        tetromino.reset();
        self.hold_queue = Some(tetromino);
    }

    /// Put `tetromino_in_play` in the **Hold Queue** and return whether the new `tetromino_in_play`
    /// is valid wrt the grid.
    fn hold_tetromino<R: Rng>(&mut self, rng: &mut R) -> TetrisResult {
        let previously_active = self.replace_tetromino_in_play_using_hold_queue(rng);
        self.put_in_hold_queue(previously_active);
        self.tetromino_in_play.is_valid_in_grid(&self.grid)
    }

    /// Lock down `tetromino_in_play`, and return whether game over has occurred.
    fn lock_down<R: Rng>(&mut self, rng: &mut R) -> TetrisResult {
        let previously_active = self.replace_tetromino_in_play(rng);

        let new_completed_lines = previously_active.lock_down(&mut self.grid)?;
        self.update_new_completed_lines(new_completed_lines);

        self.grid
            .apply_received_garbage(self.received_garbage_lines)?;
        self.received_garbage_lines = 0;

        self.tetromino_in_play.is_valid_in_grid(&self.grid)
    }

    /// Increase counters ([TetrisPlayer::new_completed_lines] and [TetrisPlayer::score])
    /// after `new_completed_lines` lines have been cleared.
    fn update_new_completed_lines(&mut self, new_completed_lines: u64) {
        self.new_completed_lines += new_completed_lines;
        self.score += new_completed_lines;
    }

    /// Increase the count of received garbage lines ([TetrisPlayer::received_garbage_lines]).
    ///
    /// Garbage lines will be pushed to the grid later (during **Lock Down**).
    pub fn push_garbage(&mut self, nb_completed_lines: u64) {
        self.received_garbage_lines += nb_completed_lines;
    }

    /// Return the number of lines completed since the last call [TetrisPlayer::new_completed_lines].
    pub fn get_lines_completed(&mut self) -> u64 {
        let lines = self.new_completed_lines;
        self.new_completed_lines = 0;
        lines
    }

    /// Returns a hard-dropped copy of the [TetrisPlayer::tetromino_in_play].
    pub fn get_ghost_tetromino(&self) -> Tetromino {
        let mut ghost_tetromino = self.tetromino_in_play.clone();
        ghost_tetromino.try_apply(TetrominoMove::HardDrop, &self.grid);
        ghost_tetromino
    }
}

impl Display for TetrisPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = self.grid.clone();
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
