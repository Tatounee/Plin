use chrono::Local;
use serenity::client::Context;
use serenity::model::id::ChannelId;

use crate::river_race::RiverRace;
use crate::{Day, IsNewMessage, PeriodIndex, Post};

// TODO: update the date for each post
#[macro_export]
macro_rules! write_post {
    ($m:expr, $river_race:expr, $clans_fielded:expr, $date:expr) => {
        $m.embed(|e| {
            e.color((255, 125, 0))
                .title("Avancement des combats")
                .thumbnail("https://i.imgur.com/3PpIKA2.png")
                .fields($clans_fielded)
                .footer(|f| {
                    f.text(match $river_race.period_type {
                        crate::river_race::Period::War => format!(
                            "Jour de combat n°{}",
                            $river_race.period_index - $river_race.section_index * 7 - 2
                        ),
                        crate::river_race::Period::Training => format!(
                            "Jour d'entrainement n°{}",
                            $river_race.period_index - $river_race.section_index * 7 + 1
                        ),
                    })
                })
                // .image("http://apcpedagogie.com/wp-content/uploads/2017/12/graphique-excel.jpg")
                .timestamp($date)
        })
    };
}

pub async fn send_post(
    channel: ChannelId,
    ctx: &Context,
    river_race: &RiverRace,
    clans_fielded: Vec<(&String, String, bool)>,
    date: String,
    period_index: i32,
) {
    let mut data = ctx.data.write().await;
    data.insert::<IsNewMessage>(false);
    match channel
        .send_message(&ctx.http, |m| {
            write_post!(m, river_race, clans_fielded, date)
        })
        .await
    {
        Ok(post) => {
            data.insert::<Day>(date_formated());
            data.insert::<Post>(post);
            data.insert::<PeriodIndex>(period_index)
        }
        Err(why) => println!("Error sending message: {:?}", why),
    }
}

pub async fn edit_post(
    ctx: &Context,
    river_race: &RiverRace,
    clans_fielded: Vec<(&String, String, bool)>,
    date: String,
) {
    let mut data = ctx.data.write().await;
    let post = data.get_mut::<Post>().unwrap();
    match post
        .edit(&ctx.http, |m| {
            write_post!(m, river_race, clans_fielded, date)
        })
        .await
    {
        Ok(_) => {
            data.insert::<Day>(date_formated());
        }
        Err(why) => println!("Error editing message: {:?}", why),
    }
}

#[inline]
pub fn date_formated() -> String {
    format!("{:?}", Local::now())[0..19].to_owned()
}
