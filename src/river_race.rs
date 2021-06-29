use std::collections::HashMap;

use reqwest::{header::HeaderMap, Client};
use serde::Deserialize;
use tokio::sync::mpsc;

use crate::clan::{Clan, ClanInfo};
use crate::post::Field;

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
    pub async fn clans_as_fields(&self, client: Client, header: HeaderMap) -> Vec<Field> {
        let mut clans: Vec<ClanInfo> = self.get_clan_info();

        let (tx, mut rx) = mpsc::channel(5);

        let mut clan_handler = vec![];
        for clan in clans.iter() {
            let sender = tx.clone();
            let client = client.clone();
            let header = header.clone();
            let tag = clan.tag.clone();
            clan_handler.push(tokio::task::spawn(async move {
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
            }))
        }

        drop(tx);

        let mut players = HashMap::new();
        while let Some(resp) = rx.recv().await {
            let tags = match resp.1 {
                Ok(tags) => match tags.json::<Items<Member>>().await {
                    Ok(tags) => tags
                        .into_vec()
                        .into_iter()
                        .map(|member| member.tag)
                        .collect::<Vec<String>>(),
                    Err(e) => {
                        println!("Json : {}", e);
                        continue;
                    }
                },
                Err(e) => {
                    println!("Response : {}", e);
                    continue;
                }
            };
            players.insert(resp.0, tags);
        }

        for (key, value) in players {
            clans
                .iter_mut()
                .find(|clan| clan.tag == key)
                .unwrap()
                .participants
                .retain(|tag| value.contains(tag))
        }

        clans.sort_unstable_by(|clan1, clan2| clan2.decks_used.cmp(&clan1.decks_used));
        clans.sort_unstable_by(|clan1, clan2| clan2.period_points.cmp(&clan1.period_points));
        clans
            .iter()
            .map(|clan| {
                let max_deck_usable = clan.participants.len() * 4;
                let pourcentage = (clan.decks_used as f32 / max_deck_usable as f32 * 100.) as u8;
                (
                    clan.name.to_owned(),
                    format!(
                        "⚔⠀{}/{}⠀({}%)\n🏅⠀{}",
                        clan.decks_used, max_deck_usable, pourcentage, clan.period_points
                    ),
                    true,
                )
            })
            .collect()
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
                    &clan.name,
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

impl<T> Items<T> {
    #[inline]
    fn into_vec(self) -> Vec<T> {
        self.items
    }
}
