//! Define custom Serialize for [Settings] and [PlayerScreen].
//!
//! PlayerScreen can be serialized as [MessageType::PlayerScreen](super::MessageType::PlayerScreen).
//! Settings can be serialized as [MessageType::Settings](super::MessageType::Settings).
use super::PlayerScreen;
use serde::{ser::SerializeStruct, Serialize, Serializer};

impl Serialize for PlayerScreen {
    /// Serializes this value.
    ///
    /// It is serialized as PlayerScreenMsg(self) if serialize_as_msg is set to true.
    /// Otherwise, it's serialized as it would with #[derive(Serialize)].
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !*self.serialize_as_msg.borrow() {
            let mut s = serializer.serialize_struct("PlayerScreen", 8)?;
            s.serialize_field("grid", &self.grid)?;
            s.serialize_field("score", &self.score)?;
            s.serialize_field("new_completed_lines", &self.new_completed_lines)?;
            s.serialize_field("active_tetromino", &self.active_tetromino)?;
            s.serialize_field("saved_tetromino", &self.saved_tetromino)?;
            s.serialize_field("fifo_next_tetromino", &self.fifo_next_tetromino)?;
            s.serialize_field("ghost_tetromino", &self.ghost_tetromino)?;
            s.serialize_field("serialize_as_msg", &self.serialize_as_msg)?;
            s.end()
        } else {
            {
                let mut a = self.serialize_as_msg.borrow_mut();
                *a = false;
            }
            let s = serializer.serialize_newtype_variant("MessageType", 0, "PlayerScreen", self);
            {
                let mut a = self.serialize_as_msg.borrow_mut();
                *a = true;
            }
            s
        }
    }
}
