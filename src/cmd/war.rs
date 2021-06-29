use std::str::FromStr;

use humantime::Duration;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::{channel::Message, id::ChannelId},
    utils::MessageBuilder,
};

use crate::{send, IsNewMessage, PostChannelId, Run, UpdateDuration, UpdatePost};

#[group("Guerre")]
#[commands(guerre_set, post, start, stop)]
#[prefixes("guerre", "g", "riverrace", "rr")]
pub struct War;

#[command("set")]
#[description("Définit les paramètres des posts.")]
#[sub_commands(set_channel, set_interval, set_tag)]
#[required_permissions("ADMINISTRATOR")]
async fn guerre_set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut have_channel = false;
    let mut have_interval = false;
    let mut have_tag = false;

    for arg in (0..3).filter_map(|_| args.single::<String>().ok()) {
        if !have_channel {
            if ChannelId::from_str(&arg).is_ok() {
                set_channel(&ctx, &msg, Args::new(&arg, &[])).await?;
                have_channel = true;
                continue;
            }
        }
        if !have_interval {
            if Duration::from_str(&arg).is_ok() {
                set_interval(&ctx, &msg, Args::new(&arg, &[])).await?;
                have_interval = true;
                continue;
            }
        }
        if !have_tag {
            set_tag(&ctx, &msg, Args::new(&arg, &[])).await?;
            have_tag = true;
            continue;
        }
    }
    Ok(())
}

#[command("channel")]
#[description("Définit le channel des posts.")]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
async fn set_channel(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Some(arg) = args.current() {
        if let Ok(channel_id) = ChannelId::from_str(arg) {
            if ctx.cache.guild_channel(channel_id).await.is_some() {
                let mut data = ctx.data.write().await;
                data.insert::<PostChannelId>(channel_id);
                send!(
                    msg.reply(
                        &ctx,
                        MessageBuilder::new()
                            .push("Channel des posts changé pour ")
                            .channel(channel_id),
                    )
                    .await
                );
            } else {
                send!(
                    msg.reply(
                        &ctx,
                        format!("Le channel d'id `{}` n'existe pas.", channel_id),
                    )
                    .await
                );
            }
        } else {
            send!(
                msg.reply(&ctx, format!("Le channel `{}` n'existe pas.", arg))
                    .await
            );
        }
    } else {
        send!(msg.reply(&ctx, "Aucun channel donné.").await);
    }
    Ok(())
}

#[command("interval")]
#[description("Définit l'interval de temps des mises à jours des posts.")]
#[required_permissions("ADMINISTRATOR")]
async fn set_interval(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Some(arg) = args.current() {
        match Duration::from_str(arg) {
            Ok(duration) => {
                let mut data = ctx.data.write().await;
                data.insert::<UpdateDuration>(duration.as_secs());
                send!(
                    msg.reply(
                        &ctx,
                        format!(
                            "Interval de temps des mises à jours des posts changé pour {}.",
                            duration
                        ),
                    )
                    .await
                );
            }
            Err(e) => send!(msg.reply(&ctx, format!("{}", e)).await),
        }
    } else {
        send!(msg.reply(&ctx, "Aucun interval de temps donné.").await);
    }
    Ok(())
}

#[command("tag")]
#[description("Définit le tag de clan pour le traqueur.")]
#[required_permissions("ADMINISTRATOR")]
async fn set_tag(_ctx: &Context, _msg: &Message, _args: Args) -> CommandResult {
    Ok(())
    // PAS FAIT !!!
    // if args.current().is_some() {
    //     match args.single::<Duration>() {
    //         Ok(duration) => {
    //             let data = ctx.data.write().await;
    //             data.insert::<UpdateDuration>(duration.as_secs());
    //         }
    //         Err(e) => send!(msg.reply(&ctx.http, format!("{}", e)).await),
    //     }
    // } else {
    //     send!(msg.reply(&ctx.http, "Aucun tag de clan donné.").await);
    // }
}

#[command("post")]
#[sub_commands(update_post, new_post)]
#[description("Outils relatifs aux posts.")]
#[required_permissions("ADMINISTRATOR")]
async fn post(_: &Context, _: &Message, _: Args) -> CommandResult {
    Ok(())
}

#[command("update")]
#[description("Met à jours le post actuel.")]
#[bucket = "update_tracker"]
#[required_permissions("ADMINISTRATOR")]
async fn update_post(ctx: &Context, _: &Message, _: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    data.insert::<UpdatePost>(true);
    Ok(())
}

#[command("new")]
#[description("Créer un nouveau post (ne met plus à jour l'ancien).")]
#[required_permissions("ADMINISTRATOR")]
async fn new_post(ctx: &Context, _: &Message, _: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    data.insert::<UpdatePost>(true);
    data.insert::<IsNewMessage>(true);
    Ok(())
}

#[command("start")]
#[description("Démare le traqueur des combats de guerre.")]
#[required_permissions("ADMINISTRATOR")]
async fn start(ctx: &Context, _: &Message, _: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    data.insert::<Run>(true);
    Ok(())
}

#[command("stop")]
#[description("Stop le traqueur des combats de guerre.")]
#[required_permissions("ADMINISTRATOR")]
async fn stop(ctx: &Context, _: &Message, _: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    data.insert::<Run>(false);
    Ok(())
}
