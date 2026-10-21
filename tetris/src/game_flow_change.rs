use crate::settings::Settings;

/// Indicates whether the player commands lead the game to pause, resume, restart or no.
/// The GameOver variant is only used for remote players.
#[derive(Debug, PartialEq)]
pub enum GameFlowChange {
    Restart,
    Resume,
    Pause,
    GameOver,
    Sync(Settings),
    Hello(String),
    NoChange,
}