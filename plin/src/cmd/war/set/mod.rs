mod channel;
mod interval;
mod tag;

use std::str::FromStr;

use humantime::Duration;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::ChannelId},
};

use channel::{set_channel, SET_CHANNEL_CMD_COMMAND};
use interval::{set_interval, SET_INTERVAL_CMD_COMMAND};
use tag::{set_tag, SET_TAG_CMD_COMMAND};

use crate::send;

#[command("set")]
#[min_args(1)]
#[max_args(3)]
#[description("Définit les paramètres des posts du traqueur.")]
#[sub_commands(set_channel_cmd, set_interval_cmd, set_tag_cmd)]
#[required_permissions("ADMINISTRATOR")]
async fn war_set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut have_channel = false;
    let mut have_interval = false;
    let mut have_tag = false;

    let guild_id = msg.guild_id.unwrap();
    let mut reply_buffer = Vec::new();

    for arg in (0..3).filter_map(|_| args.single::<String>().ok()) {
        if !have_channel && ChannelId::from_str(&arg).is_ok() {
            reply_buffer.push(set_channel(ctx, &guild_id, &arg).await?);
            have_channel = true;
            continue;
        }
        if !have_interval && Duration::from_str(&arg).is_ok() {
            reply_buffer.push(set_interval(ctx, &guild_id, &arg).await?);
            have_interval = true;
            continue;
        }
        if !have_tag {
            reply_buffer.push(set_tag(ctx, &guild_id, &arg).await?);
            have_tag = true;
            continue;
        }
    }

    send!(msg.reply(ctx, reply_buffer.join("\n")).await);

    Ok(())
}
