#![deny(missing_docs)]

//! This library crate provides core functionality for the tetris game.
//! It aims to follow the [2009 Tetris Guideline](<https://ia800405.us.archive.org/12/items/2009-tetris-variant-concepts_202201/2009%20Tetris%20Design%20Guideline.pdf>)
//! as closely as possible.
//!
//! # Scope
//!
//! This crate provides pure logic (no OS interaction), it's meant to be used by a tetris engine that will handle:
//! - rendering (this crate provides the [RenderTetrisCore] trait)
//! - player input management (this crate expects [TetrisCommand]s as input)
//! - time management (this crate doesn't know the time)
//! - random generation (this crates only uses [MockRng] for testing and doc purposes)
//! - player interactions: sending garbage from one player to another ([TetrisPlayer] is serializable and has methods to handle garbage)
//!
//! The restricted responsibilities of this crate mean everything is extensively testable,
//! and the goal is indeed to test as much of the code as possible and make it super reliable.
//!
//! # Functionalities
//!
//! This crate has the following functionalities:
//! - support for the 6 classic tetromino movements (left, right, clockwise, counter-clockwise, soft drop, hard drop), and additionally 180° turns
//! - wallkick support using the Super Rotation System (when you try to rotate a tetromino next to a wall,
//!   the regular move might be impossible but SRS will first translate the tetromino to make the rotation succeed)
//! - support for the Hold Queue
//! - support for sending/receiving garbage
//! - all 3 game over conditions are supported (see [GameOverError])
//! - customization of the tetris grid size, of the tetromino bags
//!
//! # Structure
//!
//! This crate provides `struct` [TetrisPlayer] and `enum` [TetrisCommand], which can be used
//! to represent the state of a tetris player and its changes. It also provides `struct` [MockRng] to
//! mock the random generation, and `trait` [RenderTetrisCore] to render a [TetrisPlayer].
//!
#![doc = simple_mermaid::mermaid!("core_tetris.mmd")]
//!
//! # Examples
//!
//! ```
//! # use core_tetris::{TetrisPlayer, TetrisCommand, TetrominoMove, TetrominoKind, MockRng, BagType};
//! #
//! let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::T, TetrominoKind::O]);
//! let mut garbage_rng = MockRng::right_aligned_garbage();
//! let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);
//! ```
//! Once created, a [TetrisPlayer] can be controlled with [TetrisCommand]s.
//!
//! Let's look at the results of some commands by printing [TetrisPlayer].
//! Printing [TetrisPlayer] will only show the grid contents and the active tetromino, but [TetrisPlayer]
//! also stores the **Hold Queue**, the **Next Queue**, the score, etc.
//!
//! ```
//! # use core_tetris::{TetrisPlayer, GameOverError, TetrisCommand, TetrominoMove, TetrominoKind, MockRng, BagType};
//! #
//! # let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::T, TetrominoKind::O]);
//! # let mut garbage_rng = MockRng::right_aligned_garbage();
//! let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);
//! assert_eq!(player.to_string(), concat!(
//!         "---------\n",
//!         "    T    \n", // Buffer Zone
//!         "   TTT   \n", // the first tetromino is a T, as specified by `rng`
//!         "---------\n", // Skyline
//!         "         \n", // Matrix
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "---------\n",
//!     ));
//! player.try_apply(TetrominoMove::Left.into(), &mut rng, &mut garbage_rng)?;
//! assert_eq!(player.to_string(), concat!(
//!         "---------\n",
//!         "   T     \n",
//!         "  TTT    \n",
//!         "---------\n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "---------\n",
//!     ));
//! player.try_apply(TetrominoMove::Clockwise.into(), &mut rng, &mut garbage_rng)?;
//! assert_eq!(player.to_string(), concat!(
//!         "---------\n",
//!         "   T     \n",
//!         "   TT    \n",
//!         "---------\n",
//!         "   T     \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "---------\n",
//!     ));
//! player.try_apply(TetrominoMove::HardDrop.into(), &mut rng, &mut garbage_rng)?;
//! assert_eq!(player.to_string(), concat!(
//!         "---------\n",
//!         "    OO   \n", // the second tetromino is an O, as specified by `rng`
//!         "    OO   \n",
//!         "---------\n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "   T     \n",
//!         "   TT    \n",
//!         "   T     \n",
//!         "---------\n",
//!     ));
//! # Ok::<(), GameOverError>(())
//! ```
//! NB: [TetrominoMove::HardDrop] should be used to lock down the tetromino in play some time after it reaches the bottom.
//! This crate doesn't know the time, so it doesn't know when it's the right time to do that.
//! Likewise, this crate wouldn't know when to call [TetrominoMove::Fall] to make the active tetromino
//! fall towards the bottom, it's the job of the tetris engine to regularly call [TetrominoMove::Fall].
//! ```
//! # use core_tetris::{TetrisPlayer, GameOverError, TetrisCommand, TetrominoMove, TetrominoKind, MockRng, BagType};
//! #
//! # let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::T, TetrominoKind::O]);
//! # let mut garbage_rng = MockRng::right_aligned_garbage();
//! # let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);
//! # player.try_apply(TetrisCommand::Move(TetrominoMove::Left), &mut rng, &mut garbage_rng);
//! # player.try_apply(TetrisCommand::Move(TetrominoMove::Clockwise), &mut rng, &mut garbage_rng);
//! # player.try_apply(TetrisCommand::Move(TetrominoMove::HardDrop), &mut rng, &mut garbage_rng);
//! player.try_apply(TetrisCommand::Hold, &mut rng, &mut garbage_rng)?;
//! assert_eq!(player.to_string(), concat!(
//!         "---------\n",
//!         "    T    \n", // a new tetromino is automatically addded to the grid
//!         "   TTT   \n", // it's a T, as specified by `rng`
//!         "---------\n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "   T     \n",
//!         "   TT    \n",
//!         "   T     \n",
//!         "---------\n",
//!     ));
//! player.push_garbage(1);
//! assert_eq!(player.to_string(), concat!(
//!         "---------\n",
//!         "    T    \n",
//!         "   TTT   \n",
//!         "---------\n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "   T     \n",
//!         "   TT    \n",
//!         "   T     \n",
//!         "---------\n",
//!     ));
//! # Ok::<(), GameOverError>(())
//! ```
//! [TetrisPlayer::push_garbage] doesn't have an immediate result, garbage can only be added to the grid
//! during the lock down stage. Let's trigger one to see what happens.
//! ```
//! # use core_tetris::{TetrisPlayer, GameOverError, TetrisCommand, TetrominoMove, TetrominoKind, MockRng, BagType};
//! #
//! # let mut rng = MockRng::tetromino_cycle(&[TetrominoKind::T, TetrominoKind::O]);
//! # let mut garbage_rng = MockRng::right_aligned_garbage();
//! # let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);
//! # player.try_apply(TetrisCommand::Move(TetrominoMove::Left), &mut rng, &mut garbage_rng);
//! # player.try_apply(TetrisCommand::Move(TetrominoMove::Clockwise), &mut rng, &mut garbage_rng);
//! # player.try_apply(TetrisCommand::Move(TetrominoMove::HardDrop), &mut rng, &mut garbage_rng);
//! # player.try_apply(TetrisCommand::Hold, &mut rng, &mut garbage_rng);
//! # player.push_garbage(1);
//! player.try_apply(TetrisCommand::Move(TetrominoMove::HardDrop), &mut rng, &mut garbage_rng)?;
//! assert_eq!(player.to_string(), concat!(
//!         "---------\n",
//!         "    OO   \n",
//!         "    OO   \n",
//!         "---------\n",
//!         "    T    \n",
//!         "   TTT   \n",
//!         "   T     \n",
//!         "   TT    \n",
//!         "   T     \n",
//!         " XXXXXXXX\n", // garbage is right aligned as specified in `garbage_rng`
//!         "---------\n",
//!     ));
//! # Ok::<(), GameOverError>(())
//! ```

mod circular_buffer;
mod mock_rng;
mod render_tetris_core;
mod tetris_command;
mod tetris_grid;
mod tetris_player;
mod tetromino;
mod tetromino_generator;

pub(crate) use tetromino_generator::TetrominoGenerator;

// used to update the active tetromino
pub use tetris_command::TetrisCommand;
pub use tetris_grid::{GameOverError, TetrisResult};
pub use tetromino::TetrominoMove;
// used to render the TetrisPlayer
pub use circular_buffer::CircularBuffer;
pub use mock_rng::MockRng;
pub use render_tetris_core::{RenderTetrisCore, RunningState};
pub use tetris_grid::{TetrisColor, TetrisGrid, NB_VISIBLE_BUFFER_ROWS};
pub use tetris_player::TetrisPlayer;
pub use tetromino::{Position, Tetromino, TetrominoKind};
// currently unused, TODO: propose different constructors for TetrisPlayer
pub use tetromino_generator::BagType;
