use bincode::deserialize;
use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, GuildId, MessageId};
use sled::IVec;

#[repr(transparent)]
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Id(pub u64);
impl AsRef<[u8]> for Id {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        let ptr = self as *const Self as *const u8;
        // SAFETY Self is repr(transparent) and u64 is 8 bytes wide
        unsafe { std::slice::from_raw_parts(ptr, 8) }
    }
}

impl From<GuildId> for Id {
    #[inline]
    fn from(gd: GuildId) -> Self {
        Self(gd.0)
    }
}

impl From<&GuildId> for Id {
    #[inline]
    fn from(gd: &GuildId) -> Self {
        Self(gd.0)
    }
}

impl From<ChannelId> for Id {
    #[inline]
    fn from(gd: ChannelId) -> Self {
        Self(gd.0)
    }
}

impl From<IVec> for Id {
    #[inline]
    fn from(ivec: IVec) -> Self {
        match deserialize::<Id>(ivec.as_ref()) {
            Ok(id) => id,
            Err(e) => panic!("try to deserialize IVec into Id : {:?}", e),
        }
    }
}

impl From<MessageId> for Id {
    #[inline]
    fn from(msg_id: MessageId) -> Self {
        Self(msg_id.0)
    }
}

impl From<Id> for IVec {
    #[inline]
    fn from(id: Id) -> Self {
        id.as_ref().into()
    }
}

impl From<Id> for GuildId {
    #[inline]
    fn from(id: Id) -> Self {
        Self(id.0)
    }
}

impl From<Id> for ChannelId {
    #[inline]
    fn from(id: Id) -> Self {
        Self(id.0)
    }
}
