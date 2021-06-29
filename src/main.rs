#![feature(async_closure)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

mod clan;
mod cmd;
mod data;
mod handler;
mod post;
mod river_race;

use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION},
    Client as ReqwClient,
};
use serenity::{
    framework::standard::macros::hook,
    framework::StandardFramework,
    http::Http,
    model::{channel::Message, id::ChannelId},
    utils::MessageBuilder,
    client::bridge::gateway::GatewayIntents,
    Client as DiscordClient,
};

use std::{collections::HashSet, env};

use cmd::{dev::DEV_GROUP, help::PLIN_HELP, war::WAR_GROUP};
use handler::Handler;

ctx_data!(
    UpdateDuration => u64,
    IsNewMessage => bool,
    UpdatePost => bool,
    Run => bool,
    Day => String,
    PostChannelId => ChannelId,
    Post => Message,
    ClanTag => String,
    PeriodIndex => i32,
);

const TIME_FRAGMENTATION: u64 = 5;

// handle unknow cmd

#[tokio::main]
async fn main() {
    dotenv().ok();
    let plin_token =
        env::var("PLIN_DEV_DISCORD_TOKEN").expect("Expected a Discord token in the environment");
    let cr_token =
        env::var("PLIN_CR_TOKEN").expect("Expected a Clash Royale token in the environment");

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
        .configure(|c| c.with_whitespace(true).prefixes(&["!plin ", "!pl "]).owners(owner))
        .bucket("update_tracker", |b| b.delay(TIME_FRAGMENTATION))
        .await
        .help(&PLIN_HELP)
        .group(&DEV_GROUP)
        .group(&WAR_GROUP)
        .before(
            #[hook]
            async |ctx, msg, cmd_name| {
                print!(
                    "Command = `{}`, User = `{}`, Guild = `{:?}`",
                    cmd_name,
                    msg.author.name,
                    msg.guild(&ctx.cache).await.map(|g| g.name)
                );
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
                send!(msg.reply(
                    ctx,
                    MessageBuilder::new()
                        .push("Commande inconnu: ")
                        .push_mono(unk_cmd_name),
                ).await);
            }
        );

    let mut header = HeaderMap::new();
    header.insert(ACCEPT, "application/json".parse().unwrap());
    header.insert(
        AUTHORIZATION,
        format!("Bearer {}", cr_token).parse().unwrap(),
    );
    let client = ReqwClient::new();

    let handler = Handler::new(client, header);

    let mut bot = DiscordClient::builder(&plin_token)
        .event_handler(handler)
        .framework(framework)
        .intents(
            GatewayIntents::GUILDS |
            GatewayIntents::GUILD_PRESENCES |
            GatewayIntents::GUILD_MESSAGES
        )
        .await
        .expect("Err creating client");

    if let Err(why) = bot.start().await {
        println!("Client error: {:?}", why);
    }
}
