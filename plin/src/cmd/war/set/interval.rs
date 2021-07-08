use std::str::FromStr;

use humantime::Duration;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::GuildId},
    utils::MessageBuilder,
};

use crate::data::{fields_name::*, write_guild_data};
use crate::{send, TIME_MIN};

#[command("interval")]
#[num_args(1)]
#[description("Définit l'interval de temps des mises à jours des posts.")]
#[required_permissions("ADMINISTRATOR")]
async fn set_interval_cmd(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Some(arg) = args.current() {
        send!(
            msg.reply(ctx, set_interval(ctx, &msg.guild_id.unwrap(), arg).await?)
                .await
        )
    }
    Ok(())
}

pub async fn set_interval(ctx: &Context, guild_id: &GuildId, arg: &str) -> CommandResult<String> {
    Ok(match Duration::from_str(arg) {
        Ok(duration) => {
            let interval = duration.as_secs();

            if interval >= TIME_MIN {
                write_guild_data(ctx.data.clone(), guild_id, UpdateInterval(interval)).await;
                format!(
                    "Interval de temps des mises à jours des posts changé pour {}.",
                    duration
                )
            } else {
                MessageBuilder::new()
                    .push("Interval de temps trop petit. Interval minimum: s")
                    .push_mono(TIME_MIN)
                    .push(".")
                    .build()
            }
        }
        Err(e) => format!("{}", e),
    })
}
