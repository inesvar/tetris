#![doc = simple_mermaid::mermaid!("tetris_core.mmd")]
//! This library crate provides core functionalities for a tetris game,
//! the aim is to follow the **Tetris Guideline** as closely as possible.
//!
//! The crate namely provides `struct` [TetrisPlayer] and `enum` [TetrisCommand],
//! which can be used by a tetris engine to implement the tetris game.
//! This crate has no notion of time, it's pure logic (no OS interaction).
//!
//! [TetrisPlayer] is serializable and can be sent through the network
//! to implement multi-player tetris games.
//!
//! # Examples
//!
//! ```
//! # use tetris_core::{TetrisPlayer, TetrisCommand, TetrominoMove, MockRng, BagType};
//! # let mut rng = MockRng::default();
//! let mut player = TetrisPlayer::default(&mut rng, BagType::default());
//! player.start();
//! player.try_apply(TetrisCommand::Move(TetrominoMove::Left), &mut rng);
//! player.try_apply(TetrisCommand::Move(TetrominoMove::Clockwise), &mut rng);
//! player.try_apply(TetrisCommand::Hold, &mut rng);
//! player.push_garbage(4);
//! let lines_to_send = player.get_lines_completed();
//! ```
//!
//! Since [TetrisPlayer] has no notion of time, it can't call [TetrisCommand::LockDown]
//! 0.2 seconds after the tetromino in play can't move down anymore.
//! It's the responsibility of the tetris engine to call [TetrisCommand::LockDown]
//! in this situation.
//!
//! However, the tetromino in play is locked down *automatically* after a hard drop,
//! as this should happen instantaneously.

mod circular_buffer;
mod tetris_grid;
mod tetris_player;
mod tetromino;
mod tetromino_generator;

pub(crate) use circular_buffer::CircularBuffer;
pub(crate) use tetromino::TetrominoKind;
pub(crate) use tetromino_generator::TetrominoGenerator;

// used to update the active tetromino
pub use tetris_grid::{GameOverError, TetrisResult};
pub use tetris_player::TetrisCommand;
pub use tetromino::TetrominoMove;
// used to render the TetrisPlayer
pub use tetris_grid::{TetrisColor, TetrisGrid, NB_VISIBLE_BUFFER_ROWS};
pub use tetris_player::TetrisPlayer;
pub use tetromino::{Position, Tetromino};
// currently unused, TODO: propose different constructors for TetrisPlayer
pub use circular_buffer::MockRng;
pub use tetromino_generator::BagType;
