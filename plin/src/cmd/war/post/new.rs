use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::{
    data::{fields_name::IsNewMessage, get_guild_data, write_unique_guild_data},
    utils::PrintPass,
};

#[command("new")]
#[num_args(0)]
#[description("Créer un nouveau post (ne met plus à jour l'ancien).")]
#[required_permissions("ADMINISTRATOR")]
async fn new_post(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    write_unique_guild_data(ctx.data.clone(), &msg.guild_id.unwrap(), IsNewMessage(true)).await;

    let rx = get_guild_data(ctx.data.clone(), &msg.guild_id.unwrap(), |gd| {
        gd.sleep_cancel.clone()
    })
    .await
    .unwrap();
    rx.send(()).await.println_and_pass();
    Ok(())
}
