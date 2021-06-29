use reqwest::{header::HeaderMap, Client};
use serenity::{async_trait, model::gateway::Ready, prelude::*};
use tokio::time::sleep;

use std::time::Duration;

use crate::post::{edit_post, send_post};
use crate::{post::date_formated, river_race::*};
use crate::{
    ClanTag, Day, IsNewMessage, PeriodIndex, PostChannelId, Run, UpdateDuration, UpdatePost,
    TIME_FRAGMENTATION,
};

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
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let mut data = ctx.data.write().await;
        data.insert::<UpdateDuration>(60 * 10);
        data.insert::<Run>(false);
        data.insert::<UpdatePost>(false);
        data.insert::<IsNewMessage>(false);
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
                                || *data.get::<UpdatePost>().unwrap()
                            {
                                drop(data);
                                let mut data = ctx.data.write().await;
                                data.insert::<UpdatePost>(false);
                                continue 'app;
                            }
                        }
                    }
                }
            }
        }
    }
}
