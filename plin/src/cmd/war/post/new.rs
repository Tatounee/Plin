use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::data::{fields_name::*, write_unique_guild_datas};

#[command("new")]
#[num_args(0)]
#[description("Créer un nouveau post (ne met plus à jour l'ancien).")]
#[required_permissions("ADMINISTRATOR")]
async fn new_post(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    write_unique_guild_datas(
        ctx.data.clone(),
        &msg.guild_id.unwrap(),
        &[UpdatePost(true), IsNewMessage(true)],
    )
    .await;
    Ok(())
}
