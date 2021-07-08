use std::str::FromStr;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{
        channel::Message,
        id::{ChannelId, GuildId},
    },
    utils::MessageBuilder,
};

use crate::{
    data::{fields_name::PostChannelId, write_guild_data},
    send,
};

#[command("channel")]
#[num_args(1)]
#[description("Définit le channel des posts.")]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
async fn set_channel_cmd(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Some(arg) = args.current() {
        send!(
            msg.reply(ctx, set_channel(ctx, &msg.guild_id.unwrap(), arg).await?)
                .await
        )
    }
    Ok(())
}

pub async fn set_channel(ctx: &Context, guild_id: &GuildId, arg: &str) -> CommandResult<String> {
    Ok(if let Ok(channel_id) = ChannelId::from_str(arg) {
        if ctx.cache.guild_channel(channel_id).await.is_some() {
            write_guild_data(
                ctx.data.clone(),
                guild_id,
                PostChannelId(Some(channel_id.into())),
            )
            .await;
            MessageBuilder::new()
                .push("Salon des posts changé pour ")
                .channel(channel_id)
                .push(".")
                .build()
        } else {
            format!("Le channel d'id `{}` n'existe pas.", channel_id)
        }
    } else {
        format!("Le channel `{}` n'existe pas.", arg)
    })
}
