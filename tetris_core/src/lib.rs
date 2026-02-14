//! This library crate provides core functionality for the tetris game.
//! It aims to follow the [2009 Tetris Guideline](<https://ia800405.us.archive.org/12/items/2009-tetris-variant-concepts_202201/2009%20Tetris%20Design%20Guideline.pdf>)
//! as closely as possible.
//! This crate provides pure logic (no OS interaction), it's meant to be used by a tetris engine that will handle:
//! - rendering
//! - player input management (this crate expects [TetrisCommand]s as input)
//! - time management (this crate doesn't know the time)
//! - random generation (this crates only uses [MockRng] for testing and doc purposes)
//! - player interactions: sending garbage from one player to another ([TetrisPlayer] has [TetrisPlayer::push_garbage] and [TetrisPlayer::get_lines_completed] methods)
//!
//! This crate has the following functionalities:
//! - support for the 6 classic tetromino movements (left, right, clockwise, counter-clockwise, soft drop, hard drop), and additionally 180° turns
//! - wallkick support using the Super Rotation System (when you try to rotate a tetromino next to a wall,
//!   the regular move might be impossible but SRS will first translate the tetromino to make the rotation succeed)
//! - support of the Hold Queue
//! - support for sending/receiving garbage
//! - all 3 game over conditions are supported (see [GameOverError])
//! - customization of the tetris grid size, of the tetromino bags
//!
//! The restricted responsibilities of this crate mean everything is extensively testable,
//! and the goal is indeed to test as much of the code as possible and make it super reliable.
//!
//! This crate provides `struct` [TetrisPlayer] and `enum` [TetrisCommand], which can be used
//! to represent the state of a tetris player and its changes. It also provides `struct` [MockRng] to
//! mock the random generation ([MockRng] is intended to be used with [BagType::NoBag]).
//!
//! [TetrisPlayer] is serializable and can be sent through the network
//! to implement multi-player tetris games.
//!
#![doc = simple_mermaid::mermaid!("tetris_core.mmd")]
//!
//! # Examples
//!
//! ```
//! # use tetris_core::{TetrisPlayer, TetrisCommand, TetrominoMove, TetrominoKind, MockRng, BagType};
//! #
//! // New tetrominos will be alternating T's and O's. Not recommended ;)
//! let mut rng = MockRng::new(vec![TetrominoKind::T, TetrominoKind::O]);
//! // `compact` creates a smaller 9x(6+2) tetris grid which is nice for printing,
//! // `default` creates a normal 10x(20+20) tetris grid and `new` lets you choose the size.
//! let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);
//! player.start(); // empty the tetris grid, put the first tetromino in the starting position
//! ```
//! Once created and initialized with `start`, a [TetrisPlayer] can be controlled with [TetrisCommand]s.
//! ```
//! # use tetris_core::{TetrisPlayer, TetrisCommand, TetrominoMove, TetrominoKind, MockRng, BagType};
//! #
//! # let mut rng = MockRng::new(vec![TetrominoKind::T, TetrominoKind::O]);
//! # // `compact` creates a smaller 9x(6+2) tetris grid which is nice for printing.
//! # // `default` creates a normal 10x(20+20) tetris grid.
//! # let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);
//! # player.start(); // empty the tetris grid, put the first tetromino in its starting position
//! player.try_apply(TetrisCommand::Move(TetrominoMove::Left), &mut rng);
//! player.try_apply(TetrisCommand::Move(TetrominoMove::Clockwise), &mut rng);
//! player.try_apply(TetrisCommand::Move(TetrominoMove::HardDrop), &mut rng);
//! player.try_apply(TetrisCommand::LockDown, &mut rng);
//! player.try_apply(TetrisCommand::Hold, &mut rng);
//! ```
//!
//! Let's look at the results of these commands by printing [TetrisPlayer] after each command.
//! Printing [TetrisPlayer] will only show the grid contents and the active tetromino, but [TetrisPlayer]
//! also stores the **Hold Queue**, the **Next Queue**, the score, etc.
//!
//! ```
//! # use tetris_core::{TetrisPlayer, TetrisCommand, TetrominoMove, TetrominoKind, MockRng, BagType};
//! #
//! # let mut rng = MockRng::new(vec![TetrominoKind::T, TetrominoKind::O]);
//! # // `compact` creates a smaller 9x(6+2) tetris grid which is nice for printing.
//! # // `default` creates a normal 10x(20+20) tetris grid.
//! # let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);
//! player.start(); // empty the tetris grid, put the first tetromino in its starting position
//! assert_eq!(player.to_string(), concat!(
//!         "---------\n",
//!         "   T     \n", // Buffer Zone
//!         "  TTT    \n",
//!         "---------\n", // Skyline
//!         "         \n", // Matrix
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "---------\n",
//!     ));
//! player.try_apply(TetrisCommand::Move(TetrominoMove::Left), &mut rng);
//! assert_eq!(player.to_string(), concat!(
//!         "---------\n",
//!         "  T      \n",
//!         " TTT     \n",
//!         "---------\n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "---------\n",
//!     ));
//! player.try_apply(TetrisCommand::Move(TetrominoMove::Clockwise), &mut rng);
//! assert_eq!(player.to_string(), concat!(
//!         "---------\n",
//!         "  T      \n",
//!         "  TT     \n",
//!         "---------\n",
//!         "  T      \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "---------\n",
//!     ));
//! player.try_apply(TetrisCommand::Move(TetrominoMove::HardDrop), &mut rng);
//! assert_eq!(player.to_string(), concat!(
//!         "---------\n",
//!         "   OO    \n",
//!         "   OO    \n",
//!         "---------\n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "  T      \n",
//!         "  TT     \n",
//!         "  T      \n",
//!         "---------\n",
//!     ));
//! ```
//! NB: [TetrominoMove::HardDrop] is automatically followed by [TetrisCommand::LockDown]
//! (which is namely responsible for putting the next tetromino in the grid).
//! However, [TetrisCommand::LockDown] has to be requested by the tetris engine in the case of a
//! tetromino that fell to the bottom without the use of hard drop.
//! 
//! This may be confusing at first, but the explanation is straightforward : this crate doesn't know the time,
//! so it doesn't know when [TetrisCommand::LockDown] should be called (which doesn't happen instantaneously after the
//! [Tetromino] hit the bottom). Likewise, this crate does't know when to call [TetrominoMove::Fall] to make
//! the [Tetromino] fall towards the bottom, it's the job of the tetris engine to regularly call [TetrominoMove::Fall].
//! ```
//! # use tetris_core::{TetrisPlayer, TetrisCommand, TetrominoMove, TetrominoKind, MockRng, BagType};
//! #
//! # let mut rng = MockRng::new(vec![TetrominoKind::T, TetrominoKind::O]);
//! # // `compact` creates a smaller 9x(6+2) tetris grid which is nice for printing.
//! # // `default` creates a normal 10x(20+20) tetris grid.
//! # let mut player = TetrisPlayer::compact(&mut rng, BagType::NoBag);
//! # player.start(); // empty the tetris grid, put the first tetromino in its starting position
//! # player.try_apply(TetrisCommand::Move(TetrominoMove::Left), &mut rng);
//! # player.try_apply(TetrisCommand::Move(TetrominoMove::Clockwise), &mut rng);
//! # player.try_apply(TetrisCommand::Move(TetrominoMove::HardDrop), &mut rng);
//! player.try_apply(TetrisCommand::Hold, &mut rng);
//! assert_eq!(player.to_string(), concat!(
//!         "---------\n",
//!         "   T     \n",
//!         "  TTT    \n",
//!         "---------\n",
//!         "         \n",
//!         "         \n",
//!         "         \n",
//!         "  T      \n",
//!         "  TT     \n",
//!         "  T      \n",
//!         "---------\n",
//!     ));
//! player.push_garbage(4);
//! let lines_to_send = player.get_lines_completed();
//! ```

mod circular_buffer;
mod tetris_grid;
mod tetris_player;
mod tetromino;
mod tetromino_generator;

pub(crate) use circular_buffer::CircularBuffer;
pub(crate) use tetromino_generator::TetrominoGenerator;

// used to update the active tetromino
pub use tetris_grid::{GameOverError, TetrisResult};
pub use tetris_player::TetrisCommand;
pub use tetromino::TetrominoMove;
// used to render the TetrisPlayer
pub use tetris_grid::{TetrisColor, TetrisGrid, NB_VISIBLE_BUFFER_ROWS};
pub use tetris_player::TetrisPlayer;
pub use tetromino::{Position, Tetromino, TetrominoKind};
// currently unused, TODO: propose different constructors for TetrisPlayer
pub use circular_buffer::MockRng;
pub use tetromino_generator::BagType;
