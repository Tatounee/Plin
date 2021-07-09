use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::data::{fields_name::*, write_guild_datas, Id};
use crate::send;

#[command("setup")]
#[num_args(0)]
#[description("Setup les parametères rapidement pour effectuer des tests.")]
#[required_permissions("ADMINISTRATOR")]
pub async fn setup(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    write_guild_datas(
        ctx.data.clone(),
        &guild_id,
        &[
            PostChannelId(Some(Id(862419878282657792))),
            ClanTag(Some("YLP9PVC9".to_owned())),
            UpdateInterval(10),
            Post(Box::new(None))
        ],
    )
    .await;

    send!(msg.reply(&ctx, "Dev ok !").await);
    Ok(())
}
