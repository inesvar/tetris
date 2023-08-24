//! Defines the mecanism of the game relative to one player.
//!
//! This module uses [back_end](crate::back_end) to make the tetromino move according to the player's commands.
//! It also generates new tetromino pieces and handles the queue of next pieces.
mod circular_buffer;
mod handle_key_player;
mod local_player;
mod player_screen;
mod pressed_keys;
mod update_player;

use self::{circular_buffer::CircularBuffer, pressed_keys::PressedKeys};
use crate::back_end::{TetrisGrid, Tetromino, TetrominoKind};
use crate::settings::NB_NEXT_TETROMINO;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

/// Local player contains all the informations relative to one player.
///
/// - the elements to show on screen
/// - the pressed keys
/// - the next tetromino in the queue as well as a random generator
#[derive(Serialize, Deserialize)]
pub struct LocalPlayer {
    /// player_screen contains all attributes visible on the screen
    /// like the TetrisGrid, the active Tetromino, the file of next Tetromino, etc.
    ///
    /// PlayerScreen, not LocalPlayer, is rendered.
    /// PlayerScreen, not LocalPlayer, will be sent to the remote.
    player_screen: PlayerScreen,
    /// keyboards keeps track of which keys are pressed and if they were pressed for a long time.
    keyboard: PressedKeys,
    /// freeze_frame indicates when to freeze the active_tetromino and get a new one.
    ///
    /// freeze_frame is updated when a tetromino reaches the bottom of the grid.
    freeze_frame: u64,
    bag_of_tetromino: Vec<TetrominoKind>,
    /// Whether information has to be sent to the remote or not.
    sender: bool,
    /// garbage_to_be_added is set before the update and reset during the update.
    garbage_to_be_added: u64,
    /// Random generator for the next pieces of tetromino.
    #[serde(skip, default = "new_pcg")]
    rng: Pcg32,
}

/// Player screen contains all the elements that will appear on the screen relative to one player.
#[derive(Deserialize)]
pub struct PlayerScreen {
    /// Tetris grid.
    pub grid: TetrisGrid,
    /// Number of lines cleared.
    pub score: u64,
    /// Indicates if the game is lost by the current player.
    pub game_over: bool,
    /// Is set and reset during the update resp. when lines are cleared and when data is sent to the remote players
    pub new_completed_lines: u64,
    /// The falling tetromino.
    pub active_tetromino: Tetromino,
    /// The held tetromino piece rendered in the corner.
    pub saved_tetromino: Option<Tetromino>,
    /// Next tetromino pieces rendered on the side.
    pub fifo_next_tetromino: CircularBuffer<NB_NEXT_TETROMINO, Tetromino>,
    /// The shade of the active tetromino after hard drop.
    pub ghost_tetromino: Option<Tetromino>,
    /// Flag not to be modified except in Serialize. Set to true.
    pub serialize_as_msg: RefCell<bool>,
}

/// Constructor for the random generator.
fn new_pcg() -> Pcg32 {
    Pcg32::seed_from_u64(0)
}
/// Output of [handle_key_press()](LocalPlayer::handle_key_press()) :
/// indicates whether the player commands lead the game to pause, resume, restart or no.
pub enum GameFlowChange {
    Restart,
    Resume,
    Pause,
    Other,
}
