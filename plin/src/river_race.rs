use std::collections::HashMap;
use std::error::Error as StdError;

use serde::Deserialize;
use tokio::sync::mpsc;

use crate::clan::{Clan, ClanInfo};
use crate::post::Field;
use crate::utils::new_client_and_header;

#[derive(Debug, Deserialize)]
#[serde(from = "String")]
pub enum Period {
    Training,
    War,
}

impl From<String> for Period {
    fn from(string: String) -> Self {
        match string.as_ref() {
            "warDay" => Period::War,
            _ => Period::Training,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RiverRace {
    pub clans: Vec<Clan>,
    #[serde(rename = "sectionIndex")]
    pub section_index: i32,
    #[serde(rename = "periodIndex")]
    pub period_index: i32,
    #[serde(rename = "periodType")]
    pub period_type: Period,
}

impl RiverRace {
    pub async fn get_current_river_race(
        tag: &str,
        cr_token: &str,
    ) -> Result<RiverRace, Box<dyn StdError>> {
        let (client, header) = new_client_and_header(cr_token);

        let resp = client
            .get(format!(
                "https://api.clashroyale.com/v1/clans/%23{}/currentriverrace",
                tag.replace("#", "")
            ))
            .headers(header)
            .send()
            .await?
            .text()
            .await?;

        serde_json::from_str::<RiverRace>(&resp).map_err(|e| {
            println!("Get Current River Race: {}", resp);
            e.into()
        })
    }

    pub async fn clans_as_fields(&self, cr_token: &str) -> Vec<Field> {
        let (client, header) = new_client_and_header(cr_token);

        let mut clans: Vec<ClanInfo> = self.get_clan_info();

        let (tx, mut rx) = mpsc::channel(5);

        for clan in clans.iter() {
            let sender = tx.clone();
            let client = client.clone();
            let header = header.clone();
            let tag = clan.tag.clone();
            tokio::task::spawn(async move {
                sender
                    .send((
                        tag.clone(),
                        client
                            .get(format!(
                                "https://api.clashroyale.com/v1/clans/%23{}/members",
                                tag.replace("#", "")
                            ))
                            .headers(header)
                            .send()
                            .await,
                    ))
                    .await
            });
        }

        drop(tx);

        // clan tag / members tag
        let mut players = HashMap::new();
        while let Some(resp) = rx.recv().await {
            let tags = match resp.1 {
                Ok(tags) => match tags.json::<Items<Member>>().await {
                    Ok(tags) => Vec::from(tags)
                        .into_iter()
                        .map(|member| member.tag)
                        .collect::<Vec<String>>(),
                    Err(e) => {
                        println!("Clans as Fields: {}", e);
                        continue;
                    }
                },
                Err(e) => {
                    println!("Response: {}", e);
                    continue;
                }
            };
            players.insert(resp.0, tags);
        }

        for (key, value) in players {
            // remove participant how aren't currently in these clans
            clans
                .iter_mut()
                .find(|clan| clan.tag == key)
                .unwrap()
                .participants
                .retain(|tag| value.contains(tag))
        }

        // Crown the clan witch can have the best score
        clans
            .iter_mut()
            .max_by_key(|clan| clan.maximum_points())
            .unwrap()
            .name
            .push_str("⠀👑");

        // If we are in training period, we sort them by the total of decks used
        match self.period_type {
            Period::Training => {
                clans.sort_unstable_by(|clan1, clan2| clan2.decks_used.cmp(&clan1.decks_used))
            }
            Period::War => {
                clans.sort_unstable_by(|clan1, clan2| clan2.period_points.cmp(&clan1.period_points))
            }
        }

        clans.iter().map(|clan| clan.to_field()).collect()
    }

    fn get_clan_info(&self) -> Vec<ClanInfo> {
        self.clans
            .iter()
            .map(|clan| {
                let decks_used_today = clan
                    .participants
                    .iter()
                    .map(|part| part.decks_used_today)
                    .sum::<i32>();
                ClanInfo::new(
                    clan.name.to_owned(),
                    decks_used_today,
                    clan.participants
                        .iter()
                        .map(|part| part.tag.clone())
                        .collect(),
                    clan.period_points,
                    clan.tag.clone(),
                )
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Member {
    tag: String,
}

#[derive(Debug, Deserialize)]
pub struct Items<T> {
    items: Vec<T>,
}

impl<T> From<Items<T>> for Vec<T> {
    fn from(items: Items<T>) -> Self {
        items.items
    }
}
