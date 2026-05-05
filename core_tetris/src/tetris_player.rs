//! Implements [TetrisPlayer].
use super::{
    BagType, CircularBuffer, TetrisCommand, TetrisGrid, TetrisResult, Tetromino,
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
    next_queue: CircularBuffer<Tetromino>,
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
    pub fn next_queue(&self) -> &CircularBuffer<Tetromino> {
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
        TetrisPlayer::from_matrix(rng, bag_type, TetrisGrid::default())
    }

    /// Creates a [TetrisPlayer]:
    /// - generating the first tetrominos using `rng` and `bag_type`;
    /// - creating a new **Matrix** using [TetrisGrid::compact].
    pub fn compact<R: Rng>(rng: &mut R, bag_type: BagType) -> Self {
        TetrisPlayer::from_matrix(rng, bag_type, TetrisGrid::compact())
    }

    /// Creates a [TetrisPlayer]:
    /// - generating the first tetrominos using `rng` and `bag_type`;
    /// - using `matrix` as the **Matrix** (`matrix` doesn't have to be empty).
    ///
    /// # Panics
    ///
    /// If the first tetromino can't spawn in its starting position in the **Matrix**.
    pub fn from_matrix<R: Rng>(rng: &mut R, bag_type: BagType, matrix: TetrisGrid) -> Self {
        let mut tetromino_bag = TetrominoGenerator::new(bag_type);
        let mut tetromino_in_play = tetromino_bag.get(rng);
        let next_tetrominos = tetromino_bag.get_chunk(rng, NEXT_QUEUE_MAX_SIZE);
        let next_queue = CircularBuffer::new(next_tetrominos);
        tetromino_in_play.enter_grid(&matrix);

        tetromino_in_play
            .is_valid_in_grid(&matrix)
            .expect("`matrix` should be able to contain the tetromino in play");

        TetrisPlayer {
            grid: matrix,
            score: 0,
            new_completed_lines: 0,
            tetromino_in_play,
            hold_queue: None,
            next_queue,
            tetromino_bag,
            received_garbage_lines: 0,
        }
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

    #[cfg(test)]
    /// Tries to apply [TetrominoMove::Right], returns whether the **Tetromino in Play** could be moved down.
    pub fn try_right(&mut self) -> bool {
        self.tetromino_in_play
            .try_apply(TetrominoMove::Right, &self.grid)
    }

    #[cfg(test)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MockRng, TetrominoKind};
    use rstest::rstest;

    #[rstest]
    #[case::east_to_south(TetrominoMove::Clockwise.into(), TetrominoMove::Clockwise.into(), concat!(
                "---------\n",
                "         \n",
                "         \n",
                "---------\n",
                "     IIII\n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "---------\n",
            ))]
    #[case::east_to_north(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into(), concat!(
                "---------\n",
                "         \n",
                "     IIII\n",
                "---------\n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "---------\n",
            ))]
    #[case::west_to_north(TetrominoMove::Counterclockwise.into(), TetrominoMove::Clockwise.into(), concat!(
                "---------\n",
                "         \n",
                "     IIII\n",
                "---------\n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "---------\n",
            ))]
    #[case::west_to_south(TetrominoMove::Counterclockwise.into(), TetrominoMove::Counterclockwise.into(), concat!(
                "---------\n",
                "         \n",
                "         \n",
                "---------\n",
                "     IIII\n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "---------\n",
            ))]
    fn i_off_the_right_wall(
        #[case] initial_rotation: TetrisCommand,
        #[case] rotation: TetrisCommand,
        #[case] expected: &str,
    ) -> TetrisResult {
        let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::I]);
        let mut garbage_rng = MockRng::right_aligned_garbage();
        let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

        assert_eq!(
            player.to_string(),
            concat!(
                "---------\n",
                "         \n",
                "   IIII  \n",
                "---------\n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "---------\n",
            )
        );
        player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;
        while player.try_right() {}

        assert_eq!(
            player.to_string(),
            concat!(
                "---------\n",
                "        I\n",
                "        I\n",
                "---------\n",
                "        I\n",
                "        I\n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "---------\n",
            )
        );

        player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

        assert_eq!(player.to_string(), expected);

        Ok(())
    }

    #[rstest]
    #[case::east_to_south(TetrominoMove::Clockwise.into(), TetrominoMove::Clockwise.into(), concat!(
                "---------\n",
                "         \n",
                "         \n",
                "---------\n",
                "IIII     \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "---------\n",
            ))]
    #[case::east_to_north(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into(), concat!(
                "---------\n",
                "         \n",
                "IIII     \n",
                "---------\n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "---------\n",
            ))]
    #[case::west_to_north(TetrominoMove::Counterclockwise.into(), TetrominoMove::Clockwise.into(), concat!(
                "---------\n",
                "         \n",
                "IIII     \n",
                "---------\n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "---------\n",
            ))]
    #[case::west_to_south(TetrominoMove::Counterclockwise.into(), TetrominoMove::Counterclockwise.into(), concat!(
                "---------\n",
                "         \n",
                "         \n",
                "---------\n",
                "IIII     \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "---------\n",
            ))]
    fn i_off_the_left_wall(
        #[case] initial_rotation: TetrisCommand,
        #[case] rotation: TetrisCommand,
        #[case] expected: &str,
    ) -> TetrisResult {
        let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::I]);
        let mut garbage_rng = MockRng::right_aligned_garbage();
        let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

        assert_eq!(
            player.to_string(),
            concat!(
                "---------\n",
                "         \n",
                "   IIII  \n",
                "---------\n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "---------\n",
            )
        );
        player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;
        while player.try_left() {}

        assert_eq!(
            player.to_string(),
            concat!(
                "---------\n",
                "I        \n",
                "I        \n",
                "---------\n",
                "I        \n",
                "I        \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "---------\n",
            )
        );

        player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

        assert_eq!(player.to_string(), expected);

        Ok(())
    }

    #[rstest]
    #[case::north_to_east(None, TetrominoMove::Clockwise.into(), concat!(
                "---------\n",
                "         \n",
                "         \n",
                "---------\n",
                "         \n",
                "         \n",
                "      I  \n",
                "      I  \n",
                "      I  \n",
                "      I  \n",
                "---------\n",
            ))]
    #[case::north_to_west(None, TetrominoMove::Counterclockwise.into(), concat!(
                "---------\n",
                "         \n",
                "         \n",
                "---------\n",
                "         \n",
                "         \n",
                "   I     \n",
                "   I     \n",
                "   I     \n",
                "   I     \n",
                "---------\n",
            ))]
    #[case::south_to_west(Some(TetrominoMove::HalfTurn.into()), TetrominoMove::Clockwise.into(), concat!(
                "---------\n",
                "         \n",
                "         \n",
                "---------\n",
                "         \n",
                "         \n",
                "      I  \n",
                "      I  \n",
                "      I  \n",
                "      I  \n",
                "---------\n",
            ))]
    #[case::south_to_east(Some(TetrominoMove::HalfTurn.into()), TetrominoMove::Counterclockwise.into(), concat!(
                "---------\n",
                "         \n",
                "         \n",
                "---------\n",
                "         \n",
                "         \n",
                "   I     \n",
                "   I     \n",
                "   I     \n",
                "   I     \n",
                "---------\n",
            ))]
    fn i_off_the_floor(
        #[case] initial_rotation: Option<TetrisCommand>,
        #[case] rotation: TetrisCommand,
        #[case] expected: &str,
    ) -> TetrisResult {
        let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::I]);
        let mut garbage_rng = MockRng::right_aligned_garbage();
        let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

        assert_eq!(
            player.to_string(),
            concat!(
                "---------\n",
                "         \n",
                "   IIII  \n",
                "---------\n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "---------\n",
            )
        );

        if let Some(command) = initial_rotation {
            player.try_apply(command, &mut rng, &mut garbage_rng)?;
        }
        while player.try_fall() {}

        assert_eq!(
            player.to_string(),
            concat!(
                "---------\n",
                "         \n",
                "         \n",
                "---------\n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "         \n",
                "   IIII  \n",
                "---------\n",
            )
        );

        player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

        assert_eq!(player.to_string(), expected);

        Ok(())
    }

    #[rstest]
    /// Testing that operations "moving to the right wall" and "turning 90°" are commutable,
    /// in situations where the tetromino faces east or west (this encompasses all situations that
    /// only work thanks to wall kicks).
    fn off_the_right_wall(
        #[values(
            TetrominoKind::O,
            TetrominoKind::I,
            TetrominoKind::T,
            TetrominoKind::L,
            TetrominoKind::J,
            TetrominoKind::S,
            TetrominoKind::Z
        )]
        kind: TetrominoKind,
        #[values(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into())]
        initial_rotation: TetrisCommand,
        #[values(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into())]
        rotation: TetrisCommand,
    ) -> TetrisResult {
        let mut rng = MockRng::tetromino_cycle(&[kind]);
        let mut garbage_rng = MockRng::right_aligned_garbage();
        let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

        player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;

        let mut no_wall_kick_player = player.clone();
        no_wall_kick_player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

        while player.try_right() {}
        while no_wall_kick_player.try_right() {}

        player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

        assert_eq!(player.to_string(), no_wall_kick_player.to_string());

        Ok(())
    }

    #[rstest]
    /// Testing that operations "moving to the left wall" and "turning 90°" are commutable,
    /// in situations where the tetromino faces east or west (this encompasses all situations that
    /// only work thanks to wall kicks).
    fn off_the_left_wall(
        #[values(
            TetrominoKind::O,
            TetrominoKind::I,
            TetrominoKind::T,
            TetrominoKind::L,
            TetrominoKind::J,
            TetrominoKind::S,
            TetrominoKind::Z
        )]
        kind: TetrominoKind,
        #[values(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into())]
        initial_rotation: TetrisCommand,
        #[values(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into())]
        rotation: TetrisCommand,
    ) -> TetrisResult {
        let mut rng = MockRng::tetromino_cycle(&[kind]);
        let mut garbage_rng = MockRng::right_aligned_garbage();
        let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

        player.try_apply(initial_rotation, &mut rng, &mut garbage_rng)?;

        let mut no_wall_kick_player = player.clone();
        no_wall_kick_player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

        while player.try_left() {}
        while no_wall_kick_player.try_left() {}

        player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

        assert_eq!(player.to_string(), no_wall_kick_player.to_string());

        Ok(())
    }

    #[rstest]
    /// Testing that operations "moving to the floor" and "turning 90°" are commutable,
    /// in situations where the tetromino faces north (this encompasses all situations that
    /// only work thanks to wall kicks).
    fn off_the_floor(
        #[values(
            TetrominoKind::O,
            TetrominoKind::T,
            TetrominoKind::L,
            TetrominoKind::J,
            TetrominoKind::S,
            TetrominoKind::Z
        )]
        kind: TetrominoKind,
        #[values(TetrominoMove::Clockwise.into(), TetrominoMove::Counterclockwise.into())]
        rotation: TetrisCommand,
    ) -> TetrisResult {
        let mut rng = MockRng::tetromino_cycle(&[kind]);
        let mut garbage_rng = MockRng::right_aligned_garbage();
        let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);

        let mut no_wall_kick_player = player.clone();
        no_wall_kick_player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

        if kind != TetrominoKind::O {
            match rotation {
                TetrisCommand::Move(TetrominoMove::Clockwise) => no_wall_kick_player.try_left(),
                TetrisCommand::Move(TetrominoMove::Counterclockwise) => {
                    no_wall_kick_player.try_right()
                }
                _ => unreachable!(),
            };
        }

        while player.try_fall() {}
        while no_wall_kick_player.try_fall() {}

        player.try_apply(rotation, &mut rng, &mut garbage_rng)?;

        assert_eq!(player.to_string(), no_wall_kick_player.to_string());

        Ok(())
    }
}
