use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::{read_guild_data, send};

#[command("info")]
#[num_args(0)]
#[required_permissions("ADMINISTRATOR")]
#[description("Affiche des données contextuelles de Plin.")]
pub async fn info(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    read_guild_data!(&ctx, &msg.guild_id.unwrap(), guild_data);
    send!(msg.reply(&ctx, guild_data.debug_discord()).await);
    Ok(())
}
