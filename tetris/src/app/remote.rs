//! Define the remote components for multi-player mode.
//!
//! There's a listener thread devoted for each remote player updating a remote player stub with a render method.
mod remote_player;

pub use self::remote_player::RemotePlayer;

use crate::{app::TetrisPlayer, settings::Settings};
use serde::{Deserialize, Serialize};

/// MessageType represents all different kinds of messages that can be sent.
///
/// To avoid copying uselessly, structs TetrisPlayer and Settings
/// are serialized directly into the corresponding enum variants
/// TetrisPlayerMsg { player_screen: TetrisPlayer } and
/// SettingsMsg { settings: Settings }.
///
/// Their serializations rely on the position of their fields (0 and 1)
/// in the struct so the Serialize implementations need to be updated
/// accordingly in case of change of MessageType.
#[derive(Serialize)]
pub enum OutboundMessage<'a> {
    TetrisPlayer(&'a TetrisPlayer), // not acknowledged as it's sent regularly
    Settings(&'a Settings),         // sent by the host of the room
    Restart,
    Pause,
    Resume,
    GameOver,
    Hello(String),
    Kill,
}

#[derive(Deserialize)]
pub enum InboundMessage {
    TetrisPlayer(TetrisPlayer), // not acknowledged as it's sent regularly
    Settings(Settings),         // sent by the host of the room
    Restart,
    Pause,
    Resume,
    GameOver,
    Hello(String),
    Kill,
}
