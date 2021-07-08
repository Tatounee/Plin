use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::data::{fields_name::Run, write_guild_data};
use crate::send;

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
    send!(msg.reply(ctx, "Traqueur de guerre arrêté.").await);
    Ok(())
}
