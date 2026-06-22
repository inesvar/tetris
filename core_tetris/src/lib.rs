#![deny(missing_docs)]

//! This library crate provides logic for the tetris game (no OS interaction, only logic).
//! It aims to follow the [2009 Tetris Guideline](<https://ia800405.us.archive.org/12/items/2009-tetris-variant-concepts_202201/2009%20Tetris%20Design%20Guideline.pdf>)
//! as closely as possible.
//!
//! # Scope
//!
//! This crate is meant to be used by a tetris engine that will handle:
//! - rendering (this crate provides the [render::RenderTetrisPlayer] trait)
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
//! - 6 classic tetromino moves, and additionally 180° turns (see [TetrisCommand])
//! - **Hold Queue** and associated [TetrisCommand::Hold]
//! - wallkick support using the **Super Rotation System** (when you try to rotate a tetromino next to a wall,
//!   the regular move might be impossible but SRS will first translate the tetromino to make the rotation succeed)
//! - scoring and garbage according to the 2009 Guideline (back-to-backs...)
//! - 3 official game over conditions are supported (see [GameOverError])
//! - T-spin and mini-T-spin detection
//! - customization of the tetris grid size, of the tetromino bags
//!
//! For a list of points where the implementation is different from the 2009 Guideline, see the Nitpicks section at the bottom.
//!
//! # Examples
//!
//! Let's go through a simple usage example.
//!
//! To create a [TetrisPlayer], you have to choose a [TetrisGrid] size ([TetrisPlayer::compact] creates a small 9x(6+2) grid)
//! and a random generator for the first [Tetromino]s.
//!
//! ```rust
#![doc = include_doctest!("examples/simple.rs", region = "tetris_player_creation")]
//! ```
//! Once created, a [TetrisPlayer] can be controlled with [TetrisCommand]s.
//! Printing [TetrisPlayer] only shows the grid and the active tetromino, but [TetrisPlayer]
//! also stores the **Hold Queue**, the **Next Queue**, the score, etc.
//!
//! ```rust
#![doc = include_doctest!("examples/simple.rs", region = "tetris_player_basic_commands")]
//! ```
//! NB: [TetrisCommand::HardDrop] should be used to lock down the tetromino in play some time after it reaches the bottom.
//! This crate doesn't know the time, so it doesn't know when it's the right time to do that.
//! Likewise, this crate wouldn't know when to call [TetrisCommand::SoftDrop] to make the active tetromino
//! fall towards the bottom, it's the job of the tetris engine to regularly call [TetrisCommand::SoftDrop].
//!
//! ```rust
#![doc = include_doctest!("examples/simple.rs", region = "tetris_player_hold_command")]
//! ```
//! [TetrisPlayer::push_garbage] doesn't have an immediate result, garbage can only be added to the grid
//! during the lock down stage. Let's trigger one to see what happens.
//!
//! ```rust
#![doc = include_doctest!("examples/simple.rs", region = "tetris_player_garbage")]
//! ```
//!
//! # Nitpicks
//!
//! The code is mostly based on the 2009 Guideline, but there are several places where it's not the case, for various reasons.
//!
//! - I didn't bother yet / personally there was no point for my use case
//!     - T tetromino wallkicks are not exactly correct, the 2009 Guideline removes the 3rd wallkick in some situations
//!     - levels, the Guideline has detailed instructions about how levels affect scoring and fall speed
//! - the 2009 seemed to have forgotten
//!     - mini-T-spins double, so I invented all the score and garbage values
//! - I might have misunderstood the 2009 Guideline
//!     - on garbage back-to-backs, I was a bit confused with the handling of the mini-T-spin single
//!       (it has no bonus, yet it can start a back-to-back sequence)
//! - I absolutely wanted this unofficial feature
//!     - half-turns
//!
//! # Structure
//!
//! This crate namely provides `struct` [TetrisPlayer], `enum` [TetrisCommand] and `enum` [LineClear],
//! which represent the state of play, the player actions and the result of these actions.
//! It also provides `struct` [MockRng] to mock the random generation,
//! and `trait` [render::RenderTetrisPlayer] to render a [TetrisPlayer].
//!
#![doc = simple_mermaid::mermaid!("core_tetris.mmd")]

mod circular_buffer;
mod line_clear;
mod mock_rng;
pub mod render;
mod score_manager;
mod tetris_command;
mod tetris_grid;
mod tetris_player;
mod tetromino;
mod tetromino_generator;

use doctest_file::include_doctest;
pub(crate) use line_clear::LineClearType;
pub(crate) use score_manager::ScoreManager;
pub(crate) use tetromino_generator::TetrominoGenerator;

// used to update the active tetromino
pub use line_clear::LineClear;
pub use tetris_command::TetrisCommand;
pub use tetris_grid::{GameOverError, TetrisGridCreationError, TetrisResult};
// used to render the TetrisPlayer
pub use circular_buffer::CircularBuffer;
pub use mock_rng::MockRng;
pub use tetris_grid::{TetrisColor, TetrisGrid};
pub use tetris_player::TetrisPlayer;
pub use tetromino::{Position, Tetromino, TetrominoKind};
// currently unused, TODO: propose different constructors for TetrisPlayer
pub use tetromino_generator::BagType;
