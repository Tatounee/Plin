use chrono::Local;
use serenity::client::Context;
use serenity::model::id::{ChannelId, GuildId};

use crate::data::{fields_name::*, write_guild_datas, write_unique_guild_data};
use crate::river_race::RiverRace;
use crate::UniqueGuildData;

pub type Field = (String, String, bool);

#[macro_export]
macro_rules! write_post {
    ($m:expr, $river_race:expr, $clans_fielded:expr) => {
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
                .timestamp(crate::post::date_formated())
        })
    };
}

#[macro_export]
macro_rules! send {
    ($msg:expr) => {
        if let Err(why) = $msg {
            println!("Error sending message: {:?}", why);
        }
    };
}

#[macro_export]
macro_rules! edit {
    ($msg:expr) => {
        if let Err(why) = $msg {
            println!("Error editing message: {:?}", why);
        }
    };
}

pub async fn send_post(
    channel: ChannelId,
    ctx: &Context,
    guild_id: &GuildId,
    river_race: &RiverRace,
    clans_fielded: Vec<Field>,
    period_index: i32,
) {
    // println!("+ write ({})", guild_id);
    write_unique_guild_data(ctx.data.clone(), guild_id, IsNewMessage(false)).await;

    // println!("- drop ({})", guild_id);
    match channel
        .send_message(&ctx, |m| write_post!(m, river_race, clans_fielded))
        .await
    {
        Ok(post) => {
            // println!("+ write ({})", guild_id);
            write_guild_datas(
                ctx.data.clone(),
                guild_id,
                &[Post(Box::new(Some(post))), PeriodIndex(Some(period_index))],
            )
            .await;
            // println!("- drop ({})", guild_id);
        }
        Err(why) => println!("Error sending message: {:?}", why),
    }
}

pub async fn edit_post(
    ctx: &Context,
    guild_id: &GuildId,
    river_race: &RiverRace,
    clans_fielded: Vec<Field>,
) {
    // println!("+ write... ({})", guild_id);

    let data = ctx.data.read().await;

    let mut guild_data = data
        .get::<UniqueGuildData>()
        .expect("execpt a UniqueGuildData struct")
        .get_mut(guild_id)
        .unwrap();

    match guild_data.post {
        Some(ref mut post) => {
            edit!(
                post.edit(ctx, |m| write_post!(m, river_race, clans_fielded.clone()))
                    .await
            )
        }
        None => println!("Error: Trying to edit an inexistent post"),
    }
}

#[inline]
pub fn date_formated() -> String {
    let date_time = Local::now();
    format!("{:?}", date_time - *date_time.offset())[0..19].to_owned()
}
