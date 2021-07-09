mod guild_data;
mod id;
mod post;

pub use guild_data::{
    fields::{fields_name, EditField, GuildDataEditableField},
    GuildData, PartialGuildData, DEFAULT_UPDATE_INTERVAL,
};
pub use id::Id;
pub use post::PostId;

pub const DATABASE_PATH: &str = "data/guilds";
