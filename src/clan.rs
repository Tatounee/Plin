use serde::Deserialize;

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
