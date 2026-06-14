#![deny(missing_docs)]

//! This library crate provides logic for the tetris game.
//! It aims to follow the [2009 Tetris Guideline](<https://ia800405.us.archive.org/12/items/2009-tetris-variant-concepts_202201/2009%20Tetris%20Design%20Guideline.pdf>)
//! as closely as possible.
//!
//! # Scope
//!
//! This crate provides pure logic (no OS interaction), it's meant to be used by a tetris engine that will handle:
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
//! - adding garbage and storing garbage count
//! - 3 official game over conditions are supported (see [GameOverError])
//! - T-spin detection
//! - customization of the tetris grid size, of the tetromino bags
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
//! Likewise, this crate wouldn't know when to call [TetrisCommand::Fall] to make the active tetromino
//! fall towards the bottom, it's the job of the tetris engine to regularly call [TetrisCommand::Fall].
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
