//! Define `struct` [LocalPlayer].
mod input_commands;
mod local_player;
mod update_player;

use self::input_commands::InputCommands;
pub use core_tetris::TetrisPlayer;
use rand_pcg::Pcg32;

/// [LocalPlayer] is a wrapper around [TetrisPlayer].
/// [TetrisPlayer] handles deterministic logic, while [TetrisPlayer]
/// additionally handles time and I/O.
pub struct LocalPlayer {
    player_screen: TetrisPlayer,
    keyboard: InputCommands,
    freeze_frame: u64,
    /// Whether information has to be sent to the remote or not.
    sender: bool,
    remote_ip: String,
    /// Random generator for the next pieces of tetromino.
    rng: Pcg32,
    /// Random generator for garbage gaps.
    garbage_rng: Pcg32,
}
