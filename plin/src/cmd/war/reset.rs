use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::{
    data::{fields_name::*, get_guild_data, write_guild_datas, DEFAULT_UPDATE_INTERVAL},
    utils::PrintPass,
};

#[command("reset")]
#[num_args(0)]
#[description("Réinitialise les paramètres des posts du traqueur.")]
#[required_permissions("ADMINISTRATOR")]
async fn war_reset(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    write_guild_datas(
        ctx.data.clone(),
        &msg.guild_id.unwrap(),
        &[
            PostChannelId(None),
            UpdateInterval(DEFAULT_UPDATE_INTERVAL),
            Run(false),
            ClanTag(None),
            Post(Box::new(None))
        ],
    )
    .await;
    let rx = get_guild_data(ctx.data.clone(), &msg.guild_id.unwrap(), |gd| {
        gd.sleep_cancel.clone()
    })
    .await
    .unwrap();
    rx.send(()).await.println_and_pass();
    Ok(())
}
