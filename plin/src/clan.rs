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
}

impl ClanInfo {
    pub fn new(
        name: String,
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

    #[inline]
    pub fn max_deck_usable(&self) -> usize {
        self.participants.len() * 4
    }

    #[inline]
    pub fn pourcentage(&self) -> u8 {
        (self.decks_used as f32 / self.max_deck_usable() as f32 * 100.) as u8
    }

    #[inline]
    pub fn maximum_points(&self) -> i32 {
        // the maximum of points you can gain from your four battle is 900, but to be more realistic, we consider that in average it's 800
        self.period_points + (self.max_deck_usable() as i32 - self.decks_used) * 800
    }
}

impl ClanInfo {
    pub fn to_field(&self) -> Field {
        (
            self.name.to_owned(),
            format!(
                "⚔⠀{}/{}⠀({}%)\n🏅⠀{}\n⭱⭱⠀{}",
                self.decks_used,
                self.max_deck_usable(),
                self.pourcentage(),
                self.period_points,
                self.maximum_points()
            ),
            true,
        )
    }
}
