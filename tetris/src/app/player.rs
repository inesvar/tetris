//! Define the mecanism of the game relative to one player.
//!
//! This module uses [core] to make the tetromino move according to the player's commands.
//! It also generates new tetromino pieces and handles the queue of next pieces.
mod handle_key_player;
mod keyboard_input;
mod local_player;
mod update_player;

use self::keyboard_input::KeyboardInput;
pub use core_tetris::TetrisPlayer;
use rand_pcg::Pcg32;

/// Local player contains all the informations relative to one player.
///
/// - the elements to show on screen
/// - the pressed keys
/// - the next tetromino in the queue as well as a random generator
pub struct LocalPlayer {
    /// player_screen contains all attributes visible on the screen
    ///
    /// TetrisPlayer, not LocalPlayer, is rendered.
    /// TetrisPlayer, not LocalPlayer, will be sent to the remote.
    player_screen: TetrisPlayer,
    /// keyboards keeps track of which keys are pressed and if they were pressed for a long time.
    keyboard: KeyboardInput,
    /// freeze_frame indicates when to freeze the active_tetromino and get a new one.
    ///
    /// freeze_frame is updated when a tetromino reaches the bottom of the grid.
    freeze_frame: u64,
    /// Whether information has to be sent to the remote or not.
    sender: bool,
    remote_ip: String,
    /// Random generator for the next pieces of tetromino.
    rng: Pcg32,
}
