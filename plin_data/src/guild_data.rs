pub mod fields;

use std::sync::Arc;

use bincode::{deserialize, serialize};
use futures::future::AbortHandle;
use serde::{Deserialize, Serialize};
use serenity::{http::Http, model::channel::Message, utils::MessageBuilder};
use sled::IVec;

use crate::{id::Id, post::PostId};

#[derive(Debug)]
pub struct GuildData {
    pub update_interval: u64,
    pub is_new_message: bool,
    pub update_post: bool,
    pub run: bool,
    pub post_channel_id: Option<Id>,
    pub post: Option<Message>,
    pub clan_tag: Option<String>,
    pub period_index: Option<i32>,
    pub abort: Option<AbortHandle>
}

impl Default for GuildData {
    fn default() -> Self {
        Self {
            update_interval: 60 * 10,
            is_new_message: false,
            update_post: false,
            run: false,
            post_channel_id: None,
            post: None,
            clan_tag: None,
            period_index: None,
            abort: None
        }
    }
}

impl GuildData {
    pub fn debug_discord(&self) -> String {
        MessageBuilder::new()
            .push("update_duration = ")
            .push_mono_line(self.update_interval)
            .push("is_new_message = ")
            .push_mono_line(self.is_new_message)
            .push("update_post = ")
            .push_mono_line(self.update_post)
            .push("run = ")
            .push_mono_line(self.run)
            .push("post_channel_id = ")
            .push_mono_line(format!("{:?}", self.post_channel_id))
            .push("post (id) = ")
            .push_mono_line(format!("{:?}", self.post.as_ref().map(|post| post.id)))
            .push("clan_tag = ")
            .push_mono_line(format!("{:?}", self.clan_tag))
            .push("period_index = ")
            .push_mono_line(format!("{:?}", self.period_index))
            .push("abort = ")
            .push_mono_line(format!("{:?}", self.abort))
            .build()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialGuildData {
    pub update_interval: u64,
    pub run: bool,
    pub post_channel_id: Option<Id>,
    pub post_id: Option<PostId>,
    pub clan_tag: Option<String>,
    pub period_index: Option<i32>,
}

impl PartialGuildData {
    pub async fn fecth(self, http: Arc<Http>) -> GuildData {
        let post = match self.post_id {
            Some(post) => http.get_message(post.channel.0, post.message.0).await.ok(),
            None => None,
        };

        GuildData {
            update_interval: self.update_interval,
            run: self.run,
            post_channel_id: self.post_channel_id,
            clan_tag: self.clan_tag,
            period_index: self.period_index,
            post,
            ..Default::default()
        }
    }
}

impl Default for PartialGuildData {
    fn default() -> Self {
        Self {
            update_interval: 60 * 10,
            run: false,
            post_channel_id: None,
            post_id: None,
            clan_tag: None,
            period_index: None,
        }
    }
}

impl From<GuildData> for PartialGuildData {
    fn from(gd: GuildData) -> Self {
        Self {
            update_interval: gd.update_interval,
            run: gd.run,
            post_channel_id: gd.post_channel_id,
            post_id: gd.post.map(PostId::from),
            clan_tag: gd.clan_tag,
            period_index: gd.period_index,
        }
    }
}

impl From<PartialGuildData> for IVec {
    fn from(gd: PartialGuildData) -> Self {
        serialize(&gd).unwrap().into()
    }
}

impl From<IVec> for PartialGuildData {
    #[inline]
    fn from(ivec: IVec) -> Self {
        match deserialize::<PartialGuildData>(ivec.as_ref()) {
            Ok(gd) => gd,
            Err(e) => panic!("try to deserialize IVec into PartialGuildData : {:?}", e),
        }
    }
}
