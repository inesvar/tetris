//! Implements [TetrisPlayer].
use super::{
    BagType, CircularBuffer, GameOverError, TetrisCommand, TetrisGrid, TetrisResult, Tetromino,
    TetrominoGenerator, TetrominoMove,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub const NEXT_QUEUE_MAX_SIZE: usize = 6;

/// State of a tetris player.
///
/// According to the **Tetris Guideline**, the game elements namely include :
/// - **Matrix**: "the area where game play occurs.", see [TetrisPlayer::grid].
/// - **Tetromino in Play**: "the player can manipulate this tetrimino by moving it right or left,
///   rotating it clockwise or counterclockwise, and Hard or Soft dropping it.",
///   see [TetrisPlayer::tetromino_in_play].
///   
///   NB: this implementation also supports 180° rotation.
/// - **Next Queue**: "allows the player to see the next tetrimino
///   that will be generated and put into play.", see [TetrisPlayer::next_queue]
/// - **Hold Queue**: "allows the player to “hold” a falling tetrimino for as long as they wish.",
///   see [TetrisPlayer::hold_queue]
///
/// [TetrisPlayer] is serializable and can be sent through the network
/// to implement multi-player tetris games. Note that this is not very efficient because
/// except the **Tetromino in Play**, most elements don't change between frames.
#[derive(Serialize, Deserialize, Clone)]
pub struct TetrisPlayer {
    grid: TetrisGrid,
    tetromino_in_play: Tetromino,
    next_queue: Option<CircularBuffer<Tetromino>>,
    hold_queue: Option<Tetromino>,
    /// Tetromino bag (used to refill the **Next Queue**).
    tetromino_bag: TetrominoGenerator,
    score: u64,
    new_completed_lines: u64,
    /// received_garbage_lines is set before the update and reset during the update. TODO: clarify
    received_garbage_lines: u64,
}

/// Getters and Setters.
impl TetrisPlayer {
    /// Tetris grid, or **Matrix**.
    pub fn grid(&self) -> &TetrisGrid {
        &self.grid
    }

    /// Currently falling tetromino.
    pub fn tetromino_in_play(&self) -> &Tetromino {
        &self.tetromino_in_play
    }

    /// Tetromino in the **Hold Queue**, if there's one.
    pub fn hold_queue(&self) -> &Option<Tetromino> {
        &self.hold_queue
    }

    /// Tetrominos in the **Next Queue**.
    pub fn next_queue(&self) -> &Option<CircularBuffer<Tetromino>> {
        &self.next_queue
    }

    /// Total number of lines cleared.
    pub fn score(&self) -> u64 {
        self.score
    }

    #[allow(missing_docs)]
    pub fn new_completed_lines(&self) -> u64 {
        self.new_completed_lines
    }

    #[allow(missing_docs)]
    pub fn new_completed_lines_mut(&mut self) -> &mut u64 {
        &mut self.new_completed_lines
    }

    /// Bag type.
    pub fn bag_type(&self) -> BagType {
        self.tetromino_bag.bag_type()
    }
}

/// Constructors.
impl TetrisPlayer {
    /// Creates a [TetrisPlayer]:
    /// - generating the first tetrominos using `rng` and `bag_type`;
    /// - creating a new **Matrix** using [TetrisGrid::default].
    pub fn default<R: Rng>(rng: &mut R, bag_type: BagType) -> Self {
        TetrisPlayer::try_from_matrix(rng, bag_type, TetrisGrid::default())
            .expect("there should be place for a tetromino to spawn in the grid")
    }

    /// Creates a [TetrisPlayer]:
    /// - generating the first tetrominos using `rng` and `bag_type`;
    /// - creating a new **Matrix** using [TetrisGrid::compact].
    pub fn compact<R: Rng>(rng: &mut R, bag_type: BagType) -> Self {
        TetrisPlayer::try_from_matrix(rng, bag_type, TetrisGrid::compact())
            .expect("there should be place for a tetromino to spawn in the grid")
    }

    /// Tries to create a [TetrisPlayer]:
    /// - generating the first tetrominos using `rng` and `bag_type`;
    /// - using `matrix` as the **Matrix** (`matrix` doesn't have to be empty).
    ///
    /// Fails if the first tetromino can't spawn in its starting position in the **Matrix**.
    pub fn try_from_matrix<R: Rng>(
        rng: &mut R,
        bag_type: BagType,
        matrix: TetrisGrid,
    ) -> Result<Self, GameOverError> {
        let mut tetromino_bag = TetrominoGenerator::new(bag_type);
        let mut tetromino_in_play = tetromino_bag.get(rng);
        let next_tetrominos = tetromino_bag.get_chunk(rng, NEXT_QUEUE_MAX_SIZE);
        let next_queue = CircularBuffer::new(next_tetrominos);
        tetromino_in_play.enter_grid(&matrix);

        tetromino_in_play.is_valid_in_grid(&matrix)?;

        Ok(TetrisPlayer {
            grid: matrix,
            score: 0,
            new_completed_lines: 0,
            tetromino_in_play,
            hold_queue: None,
            next_queue,
            tetromino_bag,
            received_garbage_lines: 0,
        })
    }
}

impl TetrisPlayer {
    /// Tries to apply [TetrisCommand], returns [TetrisResult] if the situation is a losing one.
    ///
    /// Refer to [TetrisCommand] documentation for more detail.
    pub fn try_apply<R1: Rng, R2: Rng>(
        &mut self,
        order: TetrisCommand,
        rng: &mut R1,
        garbage_rng: &mut R2,
    ) -> TetrisResult {
        match order {
            TetrisCommand::Move(TetrominoMove::HardDrop) => {
                self.tetromino_in_play
                    .try_apply(TetrominoMove::HardDrop, &self.grid);
                self.lock_down(rng, garbage_rng)
            }
            TetrisCommand::Move(tetromino_move) => {
                self.tetromino_in_play.try_apply(tetromino_move, &self.grid);
                Ok(())
            }
            TetrisCommand::Hold => self.hold_tetromino(rng),
        }
    }

    /// Tries to apply [TetrominoMove::Fall], returns whether the **Tetromino in Play** could be moved down.
    pub fn try_fall(&mut self) -> bool {
        self.tetromino_in_play
            .try_apply(TetrominoMove::Fall, &self.grid)
    }

    /// Tries to apply [TetrominoMove::Right], returns whether the **Tetromino in Play** could be moved down.
    pub fn try_right(&mut self) -> bool {
        self.tetromino_in_play
            .try_apply(TetrominoMove::Right, &self.grid)
    }

    /// Tries to apply [TetrominoMove::Left], returns whether the **Tetromino in Play** could be moved down.
    pub fn try_left(&mut self) -> bool {
        self.tetromino_in_play
            .try_apply(TetrominoMove::Left, &self.grid)
    }

    /// Swap `tetromino_in_play` and `swap`.
    fn swap_tetromino_in_play_with(&mut self, swap: &mut Tetromino) {
        std::mem::swap(&mut self.tetromino_in_play, swap);
        self.tetromino_in_play.enter_grid(&self.grid);
    }

    /// Get a new `tetromino_in_play` from the **Next Queue** and return the previous one.
    fn replace_tetromino_in_play<R: Rng>(&mut self, rng: &mut R) -> Tetromino {
        let mut swap = self.tetromino_bag.get(rng);

        if let Some(next_queue) = self.next_queue.as_mut() {
            next_queue.get_front_push_back(&mut swap);
        }

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
    ///
    /// This is triggered by [TetrominoMove::HardDrop].
    ///
    /// This command consists in 3 steps :
    /// - add the [TetrisPlayer::tetromino_in_play] to the [TetrisPlayer::grid] (this can fail with [GameOverError::LockOut]);
    /// - add the accumulated garbage to the [TetrisPlayer::grid] (this can fail with [GameOverError::TopOut]);
    /// - move the new [TetrisPlayer::tetromino_in_play] to its starting position (this can fail with [GameOverError::BlockOut]).
    fn lock_down<R1: Rng, R2: Rng>(&mut self, rng: &mut R1, garbage_rng: &mut R2) -> TetrisResult {
        let previously_active = self.replace_tetromino_in_play(rng);

        let new_completed_lines = previously_active.lock_down(&mut self.grid)?;
        self.update_new_completed_lines(new_completed_lines);

        self.grid
            .apply_received_garbage(self.received_garbage_lines, garbage_rng)?;
        self.received_garbage_lines = 0;

        self.tetromino_in_play.is_valid_in_grid(&self.grid)
    }

    /// Increase counters ([TetrisPlayer::new_completed_lines] and [TetrisPlayer::score])
    /// after `new_completed_lines` lines have been cleared.
    fn update_new_completed_lines(&mut self, new_completed_lines: u64) {
        self.new_completed_lines += new_completed_lines;
        self.score += new_completed_lines;
    }

    /// Increase the count of received garbage lines.
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
        for block in self.tetromino_in_play.blocks() {
            grid[block] = Some(self.tetromino_in_play.color());
        }

        write!(f, "{}", grid)
    }
}
