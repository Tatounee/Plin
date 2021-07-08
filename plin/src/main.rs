#![feature(async_closure)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]
#![feature(with_options)]

mod clan;
mod cmd;
mod data;
mod handler;
mod post;
mod river_race;
mod utils;

use bincode::deserialize;
use dashmap::DashMap;
use dotenv::dotenv;
use plin_data::PartialGuildData;
use serenity::framework::standard::DispatchError;
use serenity::{
    client::bridge::gateway::GatewayIntents, framework::standard::macros::hook,
    framework::StandardFramework, http::Http, model::id::GuildId, utils::MessageBuilder,
    Client as DiscordClient,
};
use sled::Db;

use std::collections::HashSet;
use std::convert::TryInto;
use std::env;
use std::io::stdout;
use std::io::Write;

use cmd::{dev::DEV_GROUP, help::PLIN_HELP, war::WAR_GROUP};
use data::{GuildData, DATABASE_PATH};
use handler::Handler;

use crate::utils::PrintPass;

ctx_data!(
    UniqueGuildData => DashMap<GuildId, GuildData>,
    DataBase => Db,
    CrToken => String,
);

// count command for display "typing..."

const TIME_FRAGMENTATION: u64 = 5;
const BUCKET_TIME_UPDATE_TRACKER: u64 = 60;
const TIME_MIN: u64 = 60 * 5;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let plin_token =
        env::var("PLIN_DEV_DISCORD_TOKEN").expect("Expected a Discord token in the environment");

    let http = Http::new_with_token(&plin_token);

    let owner = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owner = HashSet::new();
            owner.insert(info.owner.id);
            owner
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .prefixes(&["!plin ", "!pl ", "!"])
                .owners(owner)
        })
        .bucket("update_tracker", |b| b.delay(BUCKET_TIME_UPDATE_TRACKER))
        .await
        .before(
            #[hook]
            async |ctx, msg, cmd_name| {
                print!(
                    "Command = `{}`, User = `{}`, Guild = `{:?}`",
                    cmd_name,
                    msg.author.name,
                    msg.guild(&ctx.cache).await.map(|g| g.name)
                );
                stdout().flush().expect("Could not flush stdout");
                true
            }
        )
        .after(
            #[hook]
            async |_, _, _, cmd_result| println!(" [{:?}]", cmd_result)
        )
        .unrecognised_command(
            #[hook]
            async |ctx, msg, unk_cmd_name| {
                println!("unknow command: {}", unk_cmd_name);
                send!(
                    msg.reply(
                        ctx,
                        MessageBuilder::new()
                            .push("Commande inconnu: ")
                            .push_mono(unk_cmd_name),
                    )
                    .await
                );
            }
        )
        .on_dispatch_error(
            #[hook]
            async |ctx, msg, error| match error {
                DispatchError::Ratelimited(info) => {
                    if info.is_first_try {
                        send!(
                            msg.reply(ctx, format!("Réessayez dans {}s", info.as_secs()))
                                .await
                        )
                    }
                }
                DispatchError::CommandDisabled(_) => {
                    send!(msg.reply(ctx, "Commande désactivée.").await)
                }
                DispatchError::NotEnoughArguments { min, given } => send!(
                    msg.reply(
                        ctx,
                        format!(
                            "Pas assez d'arguments donnés. Nombre minimum: {}, nombre donné: {}.",
                            min, given
                        )
                    )
                    .await
                ),
                DispatchError::TooManyArguments { max, given } => send!(
                    msg.reply(
                        ctx,
                        format!(
                            "Trop d'arguments donnés. Nombre maximum: {}, nombre donné: {}.",
                            max, given
                        )
                    )
                    .await
                ),
                _ => (),
            }
        )
        .help(&PLIN_HELP)
        .group(&DEV_GROUP)
        .group(&WAR_GROUP);

    let handler = Handler;

    let mut bot = DiscordClient::builder(&plin_token)
        .event_handler(handler)
        .framework(framework)
        .intents(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_PRESENCES
                | GatewayIntents::GUILD_MESSAGES,
        )
        .await
        .expect("Err creating client");

    {
        let mut data = bot.data.write().await;

        let cr_token =
            env::var("PLIN_CR_TOKEN").expect("Expected a Clash Royale token in the environment");
        // If unwrap fail, we want to panic
        let database = sled::open(DATABASE_PATH).unwrap();
        let dashmap = DashMap::new();

        for (encoded_id, encoded_guild_data) in database.iter().flatten() {
            let id = GuildId(u64::from_ne_bytes(encoded_id.as_ref().try_into().unwrap()));

            match deserialize::<PartialGuildData>(encoded_guild_data.as_ref()) {
                Ok(partial_guild_data) => {
                    dashmap.insert(
                        id,
                        partial_guild_data
                            .fecth(bot.cache_and_http.http.clone())
                            .await,
                    );
                }
                Err(err) => {
                    println!(
                        "Error deserialize from database for the guild's id `{:?}`: {:?}",
                        id, err
                    );
                    database.remove(encoded_id).println_and_pass();
                }
            }
        }

        database.flush_async().await.unwrap();

        data.insert::<UniqueGuildData>(dashmap);
        data.insert::<DataBase>(database);
        data.insert::<CrToken>(cr_token);
    }

    if let Err(why) = bot.start().await {
        println!("Client error: {:?}", why);
    }
}
