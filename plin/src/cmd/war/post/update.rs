use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::{data::get_guild_data, utils::PrintPass};

#[command("update")]
#[num_args(0)]
#[description("Met à jours le post actuel.")]
#[bucket = "update_tracker"]
#[required_permissions("ADMINISTRATOR")]
async fn update_post(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let rx = get_guild_data(ctx.data.clone(), &msg.guild_id.unwrap(), |gd| {
        gd.sleep_cancel.clone()
    })
    .await
    .unwrap();
    rx.send(()).await.println_and_pass();
    Ok(())
}
