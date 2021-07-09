use serde::Deserialize;

use crate::post::Field;

#[derive(Debug, Deserialize)]
pub struct Clan {
    pub tag: String,
    pub name: String,
    pub participants: Vec<Participant>,
    #[serde(rename = "periodPoints")]
    pub period_points: i32,
}

#[derive(Debug, Deserialize)]
pub struct Participant {
    pub tag: String,
    #[serde(rename = "decksUsedToday")]
    pub decks_used_today: i32,
}

pub struct ClanInfo {
    pub name: String,
    pub decks_used: i32,
    pub participants: Vec<String>,
    pub period_points: i32,
    pub tag: String,
    pub max_deck_usable: usize,
    pub pourcentage: u8,
    pub maximum_points: i32,
}

impl ClanInfo {
    pub fn new(
        name: String,
        decks_used: i32,
        participants: Vec<String>,
        period_points: i32,
        tag: String,
    ) -> Self {
        let max_deck_usable = participants.len() * 4;
        let pourcentage = (decks_used as f32 / max_deck_usable as f32 * 100.) as u8;
        // the maximum of points you can gain from your four battle is 900, but to be more realistic, we consider that in average it's 800
        let maximum_points = period_points + (max_deck_usable as i32 - decks_used) * 800;

        Self {
            name,
            decks_used,
            participants,
            period_points,
            tag,
            max_deck_usable,
            pourcentage,
            maximum_points
        }
    }
}

impl ClanInfo {
    pub fn to_field(&self) -> Field {
        (
            self.name.to_owned(),
            format!(
                "⚔⠀{}/{}⠀({}%)\n🏅⠀{}\n⭱⭱⠀{}",
                self.decks_used, self.max_deck_usable, self.pourcentage, self.period_points, self.maximum_points
            ),
            true,
        )
    }
}
