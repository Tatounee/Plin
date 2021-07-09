use futures::future::{AbortHandle, Abortable};
use serenity::{async_trait, model::{
        gateway::{Ready, Activity},
        guild::{Guild, GuildUnavailable},
    }, prelude::*};
use tokio::{pin, select, sync::mpsc, time::sleep};

use std::sync::Arc;
use std::{env, time::Duration};

use crate::data::{
    fields_name::*, get_guild_data, remove_guild, write_unique_guild_datas, GuildData, Id,
    PartialGuildData,
};
use crate::post::{edit_post, send_post};
use crate::river_race::*;
use crate::utils::PrintPass;
use crate::{read_guild_data, DataBase, UniqueGuildData};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ctx.set_activity(Activity::competing("https://github.com/Tatounee/Plin")).await;

        let data = ctx.data.write().await;

        // If unwrap fail, we want to panic
        let database = data.get::<DataBase>().unwrap();
        let dashmap = data.get::<UniqueGuildData>().unwrap();

        for guild in ready.guilds.iter() {
            let id = Id::from(guild.id());
            if !database.contains_key(&id).unwrap() {
                println!("New guild [{:?}] added when Plin was off", id);

                database
                    .insert(id, PartialGuildData::default())
                    .println_and_pass();
                dashmap.insert(guild.id(), GuildData::default());
            }
        }

        println!("READY!")
    }

    async fn guild_delete(&self, ctx: Context, incomplete: GuildUnavailable, full: Option<Guild>) {
        let identifier = if let Some(guild) = full {
            guild.name
        } else {
            format!("{:?}", incomplete.id)
        };
        match get_guild_data(ctx.data.clone(), &incomplete.id, |gd| gd.abort.clone()).await {
            Some(abort) => abort.abort(),
            None => println!(
                "Error abording tack for the guild: {:?} [{:?}]",
                identifier, incomplete.id
            ),
        }
        remove_guild(ctx.data.clone(), &incomplete.id).await;
        println!("Guild removed: {}", identifier);
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        println!("Guild: {}, new: {}", guild.name, is_new);
        if is_new {
            let data = ctx.data.read().await;
            let database = data
                .get::<DataBase>()
                .expect("expect a GuildData struct in ctx.data");

            // If unwrap fail, we want to panic
            database
                .insert(Id::from(guild.id), PartialGuildData::default())
                .unwrap();

            database.flush_async().await.println_and_pass();

            data.get::<UniqueGuildData>()
                .unwrap()
                .insert(guild.id, GuildData::default());
        }

        let ctx = Arc::new(ctx);
        let cr_token =
            env::var("PLIN_CR_TOKEN").expect("Expected a Clash Royale token in the environment");

        let (abort_handle, abort_registration) = AbortHandle::new_pair();
        let (tx, mut rx) = mpsc::channel::<()>(5);

        write_unique_guild_datas(
            ctx.data.clone(),
            &guild.id,
            &[Abort(Some(abort_handle)), SleepCancel(Some(Arc::new(tx)))],
        )
        .await;

        let app = Abortable::new(
            async move {
                loop {
                    read_guild_data!(&ctx, &guild.id, guild_data);
                    if guild_data.run {
                        if let Some(channel) = guild_data.post_channel_id {
                            if let Some(tag) = guild_data.clan_tag.clone() {
                                drop(guild_data);

                                let river_race = match RiverRace::get_current_river_race(
                                    &tag, &cr_token,
                                )
                                .await
                                {
                                    Ok(crr) => crr,
                                    Err(e) => {
                                        println!(
                                            "Error getting river race: {} [{:?}, {}]",
                                            e, guild.id, guild.name
                                        );
                                        continue;
                                    }
                                };

                                let clans_fielded = river_race.clans_as_fields(&cr_token).await;
                                let (last_period_index_opt, is_new_message) = {
                                    read_guild_data!(&ctx, &guild.id, guild_data);
                                    (guild_data.period_index, guild_data.is_new_message)
                                };

                                if let Some(last_period_index) = last_period_index_opt {
                                    if last_period_index == river_race.period_index
                                        && !is_new_message
                                    {
                                        edit_post(&ctx, &guild.id, &river_race, clans_fielded)
                                            .await;
                                    } else {
                                        send_post(
                                            channel.into(),
                                            &ctx,
                                            &guild.id,
                                            &river_race,
                                            clans_fielded,
                                            river_race.period_index,
                                        )
                                        .await;
                                    }
                                } else {
                                    send_post(
                                        channel.into(),
                                        &ctx,
                                        &guild.id,
                                        &river_race,
                                        clans_fielded,
                                        river_race.period_index,
                                    )
                                    .await;
                                }

                                let interval = get_guild_data(ctx.data.clone(), &guild.id, |gd| {
                                    gd.update_interval
                                })
                                .await;
                                let time_off = sleep(Duration::from_secs(interval));
                                pin!(time_off);
                                let fut = rx.recv();
                                pin!(fut);

                                select! {
                                    _ = &mut time_off => (),
                                    _ = &mut fut => (),
                                }
                            }
                        }
                    }
                }
            },
            abort_registration,
        );

        tokio::spawn(app);
    }
}
