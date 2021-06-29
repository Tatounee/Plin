use reqwest::{header::HeaderMap, Client};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::*,
    utils::MessageBuilder,
};
use tokio::time::sleep;

use std::time::Duration;

use crate::cmd::*;
use crate::post::{edit_post, send_post};
use crate::{post::date_formated, river_race::*};
use crate::{
    ClanTag, Day, IsNewMessage, PeriodIndex, PostChannelId, Run, UpdateDuration, UpdatePost,
    TIME_FRAGMENTATION,
};

//* Used for testing
// static mut COUNTER: i32 = 0;
// fn river_race_dev() -> RiverRace {
//     let index = unsafe {
//         if COUNTER < 3 {
//             0
//         } else {
//             1
//         }
//     };
//     unsafe {
//         COUNTER += 1;
//         println!("COUNTER = {}", COUNTER);
//     }
//     RiverRace {
//         clans: vec![],
//         section_index: 0,
//         period_index: index,
//         period_type: Period::Training,
//     }
// }

pub struct Handler {
    client: Client,
    header: HeaderMap,
}

impl Handler {
    #[inline]
    pub fn new(client: Client, header: HeaderMap) -> Self {
        Self { client, header }
    }

    async fn get_current_river_race(&self, tag: &str) -> Result<RiverRace, reqwest::Error> {
        Ok(self
            .client
            .get(format!(
                "https://api.clashroyale.com/v1/clans/%23{}/currentriverrace",
                tag.replace("#", "")
            ))
            .headers(self.header.clone())
            .send()
            .await?
            .json::<RiverRace>()
            .await?)
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, mut msg: Message) {
        //: "!plin " => 0..6
        if msg.content.len() >= 6 && msg.content.drain(0..6).collect::<String>() == *"!plin " {
            let mut command = msg.content.trim().chars();
            let cmd = command
                .by_ref()
                .take_while(|c| *c != ' ')
                .collect::<String>();
            println!("cmd = `{}`", cmd);
            let roles = ctx
                .http
                .get_guild_roles(*msg.guild_id.unwrap().as_u64())
                .await
                .unwrap_or_else(|_| Vec::new());
            let admin_role = roles
                .iter()
                .find(|role| role.name.to_lowercase() == "admin");
            match cmd.as_ref() {
                "channel" => {
                    if (admin_role.is_some()
                        && msg
                            .author
                            .has_role(&ctx.http, admin_role.unwrap().guild_id, admin_role.unwrap())
                            .await
                            .unwrap_or(false))
                        || admin_role.is_none()
                    {
                        cmd_channel(command, &ctx, &msg).await;
                    } else if let Err(why) = msg
                            .channel_id
                            .say(&ctx.http, "Vous n'avez pas les permitions pour effectuer cette commande. Role requit : @Admin")
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                }
                "interval" => {
                    if (admin_role.is_some()
                        && msg
                            .author
                            .has_role(&ctx.http, admin_role.unwrap().guild_id, admin_role.unwrap())
                            .await
                            .unwrap_or(false))
                        || admin_role.is_none()
                    {
                        cmd_interval(command, &ctx, &msg).await;
                    } else if let Err(why) = msg
                            .channel_id
                            .say(&ctx.http, "Vous n'avez pas les permitions pour effectuer cette commande. Role requit : @Admin")
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                }
                "tag" => {
                    if (admin_role.is_some()
                        && msg
                            .author
                            .has_role(&ctx.http, admin_role.unwrap().guild_id, admin_role.unwrap())
                            .await
                            .unwrap_or(false))
                        || admin_role.is_none()
                    {
                        cmd_tag(command, &ctx, &msg).await;
                    } else if let Err(why) = msg
                            .channel_id
                            .say(&ctx.http, "Vous n'avez pas les permitions pour effectuer cette commande. Role requit : @Admin")
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                }
                "update" => {
                    if (admin_role.is_some()
                        && msg
                            .author
                            .has_role(&ctx.http, admin_role.unwrap().guild_id, admin_role.unwrap())
                            .await
                            .unwrap_or(false))
                        || admin_role.is_none()
                    {
                        update_post(&ctx).await;
                    } else if let Err(why) = msg
                            .channel_id
                            .say(&ctx.http, "Vous n'avez pas les permitions pour effectuer cette commande. Role requit : @Admin")
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                }
                "newpost" => {
                    if (admin_role.is_some()
                        && msg
                            .author
                            .has_role(&ctx.http, admin_role.unwrap().guild_id, admin_role.unwrap())
                            .await
                            .unwrap_or(false))
                        || admin_role.is_none()
                    {
                        new_post(&ctx).await;
                    } else if let Err(why) = msg
                            .channel_id
                            .say(&ctx.http, "Vous n'avez pas les permitions pour effectuer cette commande. Role requit : @Admin")
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                }
                "start" => {
                    if (admin_role.is_some()
                        && msg
                            .author
                            .has_role(&ctx.http, admin_role.unwrap().guild_id, admin_role.unwrap())
                            .await
                            .unwrap_or(false))
                        || admin_role.is_none()
                    {
                        set_run(&ctx, &msg, true, "Mise en route").await;
                    } else if let Err(why) = msg
                            .channel_id
                            .say(&ctx.http, "Vous n'avez pas les permitions pour effectuer cette commande. Role requit : @Admin")
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                }
                "stop" => {
                    if (admin_role.is_some()
                        && msg
                            .author
                            .has_role(&ctx.http, admin_role.unwrap().guild_id, admin_role.unwrap())
                            .await
                            .unwrap_or(false))
                        || admin_role.is_none()
                    {
                        set_run(&ctx, &msg, false, "Arrêt").await;
                    } else if let Err(why) = msg
                            .channel_id
                            .say(&ctx.http, "Vous n'avez pas les permitions pour effectuer cette commande. Role requit : @Admin")
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                }
                "dev" => {
                    if (admin_role.is_some()
                        && msg
                            .author
                            .has_role(&ctx.http, admin_role.unwrap().guild_id, admin_role.unwrap())
                            .await
                            .unwrap_or(false))
                        || admin_role.is_none()
                    {
                        let mut data = ctx.data.write().await;
                        data.insert::<PostChannelId>(ChannelId(827206396361441331));
                        data.insert::<ClanTag>("YLP9PVC9".to_owned());
                        data.insert::<UpdateDuration>(TIME_FRAGMENTATION);
                        msg.channel_id.say(&ctx.http, "Dev ok !").await.ok();
                    } else if let Err(why) = msg
                            .channel_id
                            .say(&ctx.http, "Vous n'avez pas les permitions pour effectuer cette commande. Role requit : @Admin")
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                }
                cmd => {
                    if let Err(why) = msg
                        .channel_id
                        .say(
                            &ctx.http,
                            MessageBuilder::new()
                                .push("Commande inconnue : ")
                                .push_mono(cmd),
                        )
                        .await
                    {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let mut data = ctx.data.write().await;
        data.insert::<UpdateDuration>(60 * 10);
        data.insert::<Run>(false);
        data.insert::<Update>(false);
        data.insert::<IsNewMessage>(true);
        drop(data);

        'app: loop {
            let data = ctx.data.read().await;
            if *data.get::<Run>().unwrap() {
                if let Some(channel) = data.get::<PostChannelId>().cloned() {
                    if let Some(tag) = data.get::<ClanTag>().cloned() {
                        let duration = *data.get::<UpdateDuration>().unwrap();
                        drop(data);

                        let river_race = match self.get_current_river_race(&tag).await {
                            Ok(crr) => crr,
                            Err(e) => {
                                println!("Error getting river race: {}", e);
                                continue;
                            }
                        };

                        let clans_fielded = river_race
                            .clans_as_fields(self.client.clone(), self.header.clone())
                            .await;

                        let data = ctx.data.read().await;
                        let last_period_index_opt = data.get::<PeriodIndex>().cloned();
                        let is_new_message = *data.get::<IsNewMessage>().unwrap();
                        let date = data.get::<Day>().cloned().unwrap_or_else(date_formated);
                        drop(data);

                        if let Some(period_index) = last_period_index_opt {
                            if period_index == river_race.period_index && !is_new_message {
                                edit_post(&ctx, &river_race, clans_fielded, date).await;
                            } else {
                                send_post(
                                    channel,
                                    &ctx,
                                    &river_race,
                                    clans_fielded,
                                    date,
                                    river_race.period_index,
                                )
                                .await;
                            }
                        } else {
                            send_post(
                                channel,
                                &ctx,
                                &river_race,
                                clans_fielded,
                                date,
                                river_race.period_index,
                            )
                            .await;
                        }

                        let repetition = duration / TIME_FRAGMENTATION;
                        let fragment = Duration::from_secs(duration / repetition);
                        let last_duration = duration;
                        for _ in 0..repetition {
                            if *ctx.data.read().await.get::<Run>().unwrap() {
                                sleep(fragment).await;
                            }
                            let data = ctx.data.read().await;
                            if *data.get::<UpdateDuration>().unwrap() != last_duration
                                || *data.get::<Update>().unwrap()
                            {
                                drop(data);
                                let mut data = ctx.data.write().await;
                                data.insert::<Update>(false);
                                continue 'app;
                            }
                        }
                    }
                }
            }
        }
    }
}
