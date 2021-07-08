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

pub struct ClanInfo<'a> {
    pub name: &'a str,
    pub decks_used: i32,
    pub participants: Vec<String>,
    pub period_points: i32,
    pub tag: String,
}

impl<'a> ClanInfo<'a> {
    pub fn new(
        name: &'a str,
        decks_used: i32,
        participants: Vec<String>,
        period_points: i32,
        tag: String,
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

impl<'a> ClanInfo<'a> {
    pub fn to_field(&self) -> Field {
        let max_deck_usable = self.participants.len() * 4;
        let pourcentage = (self.decks_used as f32 / max_deck_usable as f32 * 100.) as u8;
        (
            self.name.to_owned(),
            format!(
                "⚔⠀{}/{}⠀({}%)\n🏅⠀{}",
                self.decks_used, max_deck_usable, pourcentage, self.period_points
            ),
            true,
        )
    }
}
