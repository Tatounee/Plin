use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::data::{fields_name::UpdatePost, write_unique_guild_data};

#[command("update")]
#[num_args(0)]
#[description("Met à jours le post actuel.")]
#[bucket = "update_tracker"]
#[required_permissions("ADMINISTRATOR")]
async fn update_post(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    write_unique_guild_data(ctx.data.clone(), &msg.guild_id.unwrap(), UpdatePost(true)).await;
    Ok(())
}
