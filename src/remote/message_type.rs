use crate::{player_screen::PlayerScreen, settings::Settings};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

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
    SettingsMsg(Settings), // sent by the host of the room
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

impl Serialize for Settings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        println!("entered serializer");
        if *self.serialize_as_msg.borrow() {
            println!("second loop");
            let mut s = serializer.serialize_struct("PlayerScreen", 3)?;
            s.serialize_field("seed", &self.seed)?;
            s.serialize_field("bag_size", &self.bag_size)?;
            s.serialize_field("nb_next_tetromino", &self.nb_next_tetromino)?;
            s.serialize_field("serialized", &self.serialize_as_msg)?;
            s.end()
        } else {
            {
                let mut a = self.serialize_as_msg.borrow_mut();
                *a = false;
            }
            println!("first loop");
            println!("serializing variant");
            let s = serializer.serialize_newtype_variant("MessageType", 1, "SettingsMsg", self);
            {
                let mut a = self.serialize_as_msg.borrow_mut();
                *a = true;
            }
            s
        }
    }
}
