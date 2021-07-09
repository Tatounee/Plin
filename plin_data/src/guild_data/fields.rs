use std::sync::Arc;

use futures::future::AbortHandle;
use serenity::model::channel::Message;
use tokio::sync::mpsc::Sender;

use crate::{GuildData, Id, PartialGuildData, PostId};

#[derive(Debug, Clone)]
pub enum GuildDataEditableField {
    UpdateInterval(u64),
    IsNewMessage(bool),
    UpdatePost(bool),
    Run(bool),
    PostChannelId(Option<Id>),
    Post(Box<Option<Message>>),
    ClanTag(Option<String>),
    PeriodIndex(Option<i32>),
    Abort(Option<AbortHandle>),
    SleepCancel(Option<Arc<Sender<()>>>),
}

pub trait EditField {
    fn edit_field(&mut self, field: GuildDataEditableField);
    fn edit_fields(&mut self, fields: &[GuildDataEditableField]) {
        for field in fields {
            self.edit_field(field.clone())
        }
    }
}

impl EditField for GuildData {
    fn edit_field(&mut self, field: GuildDataEditableField) {
        match field {
            GuildDataEditableField::UpdateInterval(value) => self.update_interval = value,
            GuildDataEditableField::IsNewMessage(value) => self.is_new_message = value,
            GuildDataEditableField::UpdatePost(value) => self.update_post = value,
            GuildDataEditableField::Run(value) => self.run = value,
            GuildDataEditableField::PostChannelId(value) => self.post_channel_id = value,
            GuildDataEditableField::Post(value) => self.post = *value,
            GuildDataEditableField::ClanTag(value) => self.clan_tag = value,
            GuildDataEditableField::PeriodIndex(value) => self.period_index = value,
            GuildDataEditableField::Abort(value) => self.abort = value,
            GuildDataEditableField::SleepCancel(value) => self.sleep_cancel = value,
        }
    }
}

impl EditField for PartialGuildData {
    fn edit_field(&mut self, field: GuildDataEditableField) {
        match field {
            GuildDataEditableField::UpdateInterval(value) => self.update_interval = value,
            GuildDataEditableField::Run(value) => self.run = value,
            GuildDataEditableField::PostChannelId(value) => self.post_channel_id = value,
            GuildDataEditableField::Post(value) => self.post_id = value.map(PostId::from),
            GuildDataEditableField::ClanTag(value) => self.clan_tag = value,
            GuildDataEditableField::PeriodIndex(value) => self.period_index = value,
            _ => (),
        }
    }
}

pub mod fields_name {
    pub use super::GuildDataEditableField::{
        Abort, ClanTag, IsNewMessage, PeriodIndex, Post, PostChannelId, Run, SleepCancel,
        UpdateInterval, UpdatePost,
    };
}
