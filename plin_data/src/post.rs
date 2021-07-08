use serde::{Deserialize, Serialize};
use serenity::model::channel::Message;

use crate::id::Id;

#[derive(Debug, Deserialize, Serialize)]
pub struct PostId {
    pub channel: Id,
    pub message: Id,
}

impl From<Message> for PostId {
    #[inline]
    fn from(msg: Message) -> Self {
        Self {
            channel: Id::from(msg.channel_id),
            message: Id::from(msg.id),
        }
    }
}
