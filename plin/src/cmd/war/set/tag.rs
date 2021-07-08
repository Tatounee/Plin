use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::GuildId},
    utils::MessageBuilder,
};

use crate::data::fields_name::ClanTag;
use crate::{data::write_guild_data, utils::new_client_and_header};
use crate::{send, CrToken};

#[command("tag")]
#[num_args(1)]
#[description("Définit le tag de clan pour le traqueur.")]
#[required_permissions("ADMINISTRATOR")]
async fn set_tag_cmd(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Some(tag) = args.current() {
        send!(
            msg.reply(ctx, set_tag(ctx, &msg.guild_id.unwrap(), tag).await?)
                .await
        )
    }

    Ok(())
}

pub async fn set_tag(ctx: &Context, guild_id: &GuildId, tag: &str) -> CommandResult<String> {
    let (client, header) = new_client_and_header(ctx.data.read().await.get::<CrToken>().unwrap());

    Ok(
        if client
            .get(format!(
                "https://api.clashroyale.com/v1/clans/%23{}/currentriverrace",
                tag.replace("#", "")
            ))
            .headers(header)
            .send()
            .await?
            .text()
            .await?
            .contains("name")
        {
            write_guild_data(ctx.data.clone(), guild_id, ClanTag(Some(tag.to_owned()))).await;

            MessageBuilder::new()
                .push("Tag changer pour ")
                .push_mono(tag)
                .push(".")
                .build()
        } else {
            MessageBuilder::new()
                .push("Le tag ")
                .push_mono(tag)
                .push(" ne correspond à aucun clan.")
                .build()
        },
    )
}
