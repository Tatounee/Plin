use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::send;
use crate::{
    data::{fields_name::Run, get_guild_data, write_guild_data},
    utils::PrintPass,
};

#[command("start")]
#[num_args(0)]
#[description("Démare le traqueur des combats de guerre.")]
#[required_permissions("ADMINISTRATOR")]
async fn start(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    write_guild_data(ctx.data.clone(), &msg.guild_id.unwrap(), Run(true)).await;
    send!(msg.reply(ctx, "Traqueur de guerre mis en route !").await);
    Ok(())
}

#[command("stop")]
#[num_args(0)]
#[description("Stop le traqueur des combats de guerre.")]
#[required_permissions("ADMINISTRATOR")]
async fn stop(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    write_guild_data(ctx.data.clone(), &msg.guild_id.unwrap(), Run(false)).await;
    let rx = get_guild_data(ctx.data.clone(), &msg.guild_id.unwrap(), |gd| {
        gd.sleep_cancel.clone()
    })
    .await
    .unwrap();
    rx.send(()).await.println_and_pass();
    send!(msg.reply(ctx, "Traqueur de guerre arrêté.").await);
    Ok(())
}
