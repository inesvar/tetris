//! Defines the remote components for multi-player mode.
//!
//! There's a listener thread devoted for each remote player updating a remote player stub with a render method.
mod custom_serialize_as_msg;
mod remote_player;

pub use self::remote_player::RemotePlayer;

use crate::{app::PlayerScreen, settings::Settings};
use serde::{Deserialize, Serialize};

/// MessageType represents all different kinds of messages that can be sent.
///
/// To avoid copying uselessly, structs PlayerScreen and Settings
/// are serialized directly into the corresponding enum variants
/// PlayerScreenMsg { player_screen: PlayerScreen } and
/// SettingsMsg { settings: Settings }.
///
/// Their serializations rely on the position of their fields (0 and 1)
/// in the struct so the Serialize implementations need to be updated
/// accordingly in case of change of MessageType.
#[derive(Serialize, Deserialize)]
pub enum MessageType {
    PlayerScreenMsg(PlayerScreen), // not acknowledged as it's sent regularly
    SettingsMsg(Settings),         // sent by the host of the room
    RestartMsg,
    PauseMsg,
    ResumeMsg,
    GameOverMsg,
    HelloMsg,
    AckSettingsMsg, // sent to the host
    AckRestartMsg,
    AckPauseMsg,
    AckResumeMsg,
    AckGameOverMsg,
    AckHelloMsg,
}
