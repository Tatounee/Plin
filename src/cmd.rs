use serenity::{
    client::Context,
    model::{channel::Message, id::ChannelId},
    utils::MessageBuilder,
};

use crate::{
    ClanTag, IsNewMessage, PostChannelId, Run, Update, UpdateDuration, TIME_FRAGMENTATION,
};

#[inline]
pub async fn cmd_channel<I: Iterator<Item = char>>(command: I, ctx: &Context, msg: &Message) {
    let new_channel_u64 = command
        .skip_while(|c| *c == ' ')
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    if ctx.cache.guild_channel(new_channel_u64).await.is_some() {
        let mut data = ctx.data.write().await;
        let new_channel_id = ChannelId(new_channel_u64);
        data.insert::<PostChannelId>(new_channel_id);
        drop(data);
        new_post(ctx).await;
        if let Err(why) = msg
            .channel_id
            .say(
                &ctx.http,
                MessageBuilder::new()
                    .push("Le channel de post à été changer pour ")
                    .channel(new_channel_id),
            )
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    } else if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            format!(
                "L'id donné {} ne correspond à aucun channel",
                new_channel_u64
            ),
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}

#[inline]
pub async fn cmd_interval<I: Iterator<Item = char>>(command: I, ctx: &Context, msg: &Message) {
    let interval = command.collect::<String>().trim().to_owned();
    match interval.parse::<humantime::Duration>() {
        Ok(duration) => {
            let update_duration = duration.as_secs();
            if update_duration >= TIME_FRAGMENTATION {
                let mut data = ctx.data.write().await;
                data.insert::<UpdateDuration>(update_duration);
                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        format!(
                            "Les posts seront maintenant mise à jour tous les {}",
                            duration
                        ),
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            } else if let Err(why) = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!(
                        "Le temps donné {} est trop petit. Minimum : {}s",
                        duration, TIME_FRAGMENTATION
                    ),
                )
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
        Err(e) => {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("Erreur : {}", e))
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

#[inline]
pub async fn cmd_tag<I: Iterator<Item = char>>(command: I, ctx: &Context, msg: &Message) {
    let new_tag = command
        .skip_while(|c| *c == ' ')
        .take_while(|c| *c != ' ')
        .collect::<String>();
    let mut data = ctx.data.write().await;
    data.insert::<ClanTag>(new_tag.clone());
    drop(data);
    new_post(ctx).await;
    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, format!("Tag mise à jour pour : {}", new_tag))
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}

// TODO: use data: &RwLockWriteGuard insted of &Context
#[inline]
pub async fn new_post(ctx: &Context) {
    let mut data = ctx.data.write().await;
    data.insert::<IsNewMessage>(true);
    data.insert::<Update>(true);
}

#[inline]
pub async fn update_post(ctx: &Context) {
    let mut data = ctx.data.write().await;
    data.insert::<Update>(true);
}

#[inline]
pub async fn set_run(ctx: &Context, msg: &Message, run: bool, resp: &str) {
    let mut data = ctx.data.write().await;
    data.insert::<Run>(run);
    if let Err(why) = msg.channel_id.say(&ctx.http, resp).await {
        println!("Error sending message: {:?}", why);
    }
}
