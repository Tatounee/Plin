use core::str;
use std::collections::HashMap;

use reqwest::{header::HeaderMap, Client};
use serde::Deserialize;
use tokio::sync::mpsc;

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
    // pub state: String,
    // pub clan: Clan,
    pub clans: Vec<Clan>,
    #[serde(rename = "sectionIndex")]
    pub section_index: i32,
    #[serde(rename = "periodIndex")]
    pub period_index: i32,
    #[serde(rename = "periodType")]
    pub period_type: Period,
    // #[serde(rename = "periodLogs")]
    // pub period_logs: Vec<Log>,
}

impl RiverRace {
    pub async fn clans_as_fields(
        &self,
        client: Client,
        header: HeaderMap,
    ) -> Vec<(&String, String, bool)> {
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
                .find(|clan| clan.tag == &key)
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
                    clan.name,
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
                    clan.participants.iter().map(|part| &part.tag).collect(),
                    clan.period_points,
                    &clan.tag,
                )
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Clan {
    tag: String,
    name: String,
    participants: Vec<Participant>,
    #[serde(rename = "periodPoints")]
    period_points: i32,
    // #[serde(rename = "badgeId")]
    // badge_id: i32,
    // pub fame: i32,
    // #[serde(rename = "repairPoints")]
    // repair_points: i32,
    // #[serde(rename = "clanScore")]
    // pub clan_score: i32,
}

#[derive(Debug, Deserialize)]
pub struct Participant {
    tag: String,
    // name: String,
    // fame: i32,
    // #[serde(rename = "repairPoints")]
    // repair_points: i32,
    // #[serde(rename = "boatAttacks")]
    // boat_attacks: i32,
    // #[serde(rename = "decksUsed")]
    // decks_used: i32,
    #[serde(rename = "decksUsedToday")]
    decks_used_today: i32,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    // #[serde(rename = "periodIndex")]
// period_index: i32,
// items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    // clan: ItemClan,
// #[serde(rename = "pointsEarned")]
// points_earned: i32,
// #[serde(rename = "progressStartOfDay")]
// progress_start_of_day: i32,
// #[serde(rename = "progressEndOfDay")]
// progress_end_of_day: i32,
// #[serde(rename = "endOfDayRank")]
// end_of_day_rank: i32,
// #[serde(rename = "progressEarned")]
// progress_earned: i32,
// #[serde(rename = "numOfDefensesRemaining")]
// num_of_defenses_remaining: i32,
// #[serde(rename = "progressEarnedFromDefenses")]
// progress_earned_from_defenses: i32,
}

#[derive(Debug, Deserialize)]
pub struct ItemClan {
    // tag: String,
}

#[derive(Debug, Deserialize)]
pub struct Member {
    tag: String,
    // arena: Arena,
    // #[serde(rename = "lastSeen")]
    // last_seen: String,
    // name: String,
    // role: String,
    // #[serde(rename = "expLevel")]
    // exp_level: i32,
    // trophies: i32,
    // #[serde(rename = "clanRank")]
    // clan_rank: i32,
    // #[serde(rename = "previousClanRank")]
    // previous_clan_rank: i32,
    // donations: i32,
    // #[serde(rename = "donationsReceived")]
    // donations_received: i32,
}

#[derive(Debug, Deserialize)]
pub struct Arena {
    // name: String,
// id: i32,
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

struct ClanInfo<'a> {
    name: &'a String,
    decks_used: i32,
    participants: Vec<&'a String>,
    period_points: i32,
    tag: &'a String,
}

impl<'a> ClanInfo<'a> {
    fn new(
        name: &'a String,
        decks_used: i32,
        participants: Vec<&'a String>,
        period_points: i32,
        tag: &'a String,
    ) -> Self {
        Self {
            name,
            decks_used,
            participants,
            period_points,
            tag,
        }
    }
}
