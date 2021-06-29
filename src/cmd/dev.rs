use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{channel::Message, id::ChannelId},
    utils::MessageBuilder,
};

use crate::{
    send, ClanTag, Day, IsNewMessage, PeriodIndex, Post, PostChannelId, Run, UpdateDuration,
    UpdatePost, TIME_FRAGMENTATION,
};

#[group]
#[commands(dev_info, dev_setup)]
#[required_permissions("ADMINISTRATOR")]
#[prefixes("dev")]
pub struct Dev;

#[command("info")]
#[required_permissions("ADMINISTRATOR")]
#[description("Affiche des données contextuelles de Plin.")]
async fn dev_info(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let data = ctx.data.read().await;
    send!(
        msg.reply(
            &ctx,
            MessageBuilder::new()
                .push("UpdateDuration = ")
                .push_mono_line(data.get::<UpdateDuration>().unwrap())
                .push("IsNewMessage = ")
                .push_mono_line(data.get::<IsNewMessage>().unwrap())
                .push("Run = ")
                .push_mono_line(data.get::<Run>().unwrap())
                .push("UpdatePost = ")
                .push_mono_line(data.get::<UpdatePost>().unwrap())
                .push("PostChannelId = ")
                .push_mono_line(format!("{:?}", data.get::<PostChannelId>()))
                .push("ClanTag = ")
                .push_mono_line(format!("{:?}", data.get::<ClanTag>()))
                .push("PeriodIndex = ")
                .push_mono_line(format!("{:?}", data.get::<PeriodIndex>()))
                .push("Post = ")
                .push_mono_line(format!("{:?}", data.get::<Post>()))
                .push("Day = ")
                .push_mono(format!("{:?}", data.get::<Day>()))
        )
        .await
    );
    Ok(())
}

#[command("setup")]
#[description("Setup les parametères rapidement pour effectuer des tests.")]
#[required_permissions("ADMINISTRATOR")]
async fn dev_setup(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    data.insert::<PostChannelId>(ChannelId(827206396361441331));
    data.insert::<ClanTag>("YLP9PVC9".to_owned());
    data.insert::<UpdateDuration>(TIME_FRAGMENTATION);
    send!(msg.reply(&ctx, "Dev ok !").await);
    Ok(())
}
