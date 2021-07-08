use std::sync::Arc;

use plin_data::GuildDataEditableField;
pub use plin_data::{
    fields_name, EditField, GuildData, Id, PartialGuildData, PostId, DATABASE_PATH,
};
use serenity::{model::id::GuildId, prelude::TypeMap};
use sled::transaction::ConflictableTransactionResult;
use tokio::sync::RwLock;

use crate::{DataBase, UniqueGuildData};

#[macro_export]
macro_rules! ctx_data {
    ($($name:ident => $value:ty),* $(,)?) => {
        $(
            pub struct $name;

            impl serenity::prelude::TypeMapKey for $name {
                type Value = $value;
            }
        )*
    };
}

#[macro_export]
macro_rules! read_guild_data {
    ($ctx:expr, $guild_id:expr, $guild:ident) => {
        let data = $ctx.data.read().await;
        let unique_data = data
            .get::<crate::UniqueGuildData>()
            .expect("Exept a UniqueGuildData struct");
        let $guild = unique_data.get($guild_id).unwrap();
    };
}

pub async fn write_guild_data(
    data: Arc<RwLock<TypeMap>>,
    guild_id: &GuildId,
    new_value: GuildDataEditableField,
) {
    let data = data.read().await;
    {
        let mut guild_data = data
            .get::<UniqueGuildData>()
            .expect("execpt a UniqueGuildData struct")
            .get_mut(guild_id)
            .unwrap();
        guild_data.value_mut().edit_field(new_value.clone());
    }

    let database = data.get::<DataBase>().unwrap();

    if let Err(err) = database.transaction(|db| -> ConflictableTransactionResult<(), ()> {
        let id = Id::from(guild_id);
        let mut partial_guild_data: PartialGuildData = db.remove(id)?.unwrap().into();

        partial_guild_data.edit_field(new_value.clone());

        db.insert(id, partial_guild_data)?;
        Ok(())
    }) {
        println!("Error updating database: {:?}", err);
    }

    database.flush_async().await.unwrap();
}

pub async fn write_guild_datas(
    data: Arc<RwLock<TypeMap>>,
    guild_id: &GuildId,
    new_value: &[GuildDataEditableField],
) {
    let data = data.read().await;
    {
        let mut guild_data = data
            .get::<UniqueGuildData>()
            .expect("execpt a UniqueGuildData struct")
            .get_mut(guild_id)
            .unwrap();
        guild_data.value_mut().edit_fields(new_value);
    }

    let database = data.get::<DataBase>().unwrap();

    if let Err(err) = database.transaction(|db| -> ConflictableTransactionResult<(), ()> {
        let id = Id::from(guild_id);
        let mut partial_guild_data: PartialGuildData = db.remove(id)?.unwrap().into();

        partial_guild_data.edit_fields(new_value);

        db.insert(id, partial_guild_data)?;
        Ok(())
    }) {
        println!("Error updating database: {:?}", err);
    }

    database.flush_async().await.unwrap();
}

pub async fn write_unique_guild_datas(
    data: Arc<RwLock<TypeMap>>,
    guild_id: &GuildId,
    new_values: &[GuildDataEditableField],
) {
    let data = data.read().await;
    {
        let mut guild_data = data
            .get::<UniqueGuildData>()
            .expect("execpt a UniqueGuildData struct")
            .get_mut(guild_id)
            .unwrap();
        guild_data.edit_fields(new_values)
    }
}

pub async fn write_unique_guild_data(
    data: Arc<RwLock<TypeMap>>,
    guild_id: &GuildId,
    new_value: GuildDataEditableField,
) {
    let data = data.read().await;
    {
        let mut guild_data = data
            .get::<UniqueGuildData>()
            .expect("execpt a UniqueGuildData struct")
            .get_mut(guild_id)
            .unwrap();
        guild_data.edit_field(new_value)
    }
}

pub async fn get_guild_data<T: Clone>(
    data: Arc<RwLock<TypeMap>>,
    guild_id: &GuildId,
    get_field: impl Fn(&GuildData) -> T,
) -> T {
    let data = data.read().await;
    
    let guild_data = data
        .get::<UniqueGuildData>()
        .expect("execpt a UniqueGuildData struct")
        .get(guild_id)
        .unwrap();
    get_field(guild_data.value())
}

pub async fn remove_guild(data: Arc<RwLock<TypeMap>>, guild_id: &GuildId) {
    let data = data.read().await;
    {
        data.get::<UniqueGuildData>()
            .expect("execpt a UniqueGuildData struct")
            .remove(guild_id);
    }

    let database = data.get::<DataBase>().unwrap();
    let id = Id::from(guild_id);
    if let Err(err) = database.remove(id) {
        println!(
            "Error removing guild ({:?}) from database: {:?}",
            guild_id, err
        );
    }

    database.flush_async().await.unwrap();
}

pub async fn is_app_running(data: Arc<RwLock<TypeMap>>, guild_id: &GuildId) -> Option<bool> {
    let data = data.read().await;
    let unique_data = data
        .get::<crate::UniqueGuildData>()
        .expect("UniqueGuildData struct");
    let field = unique_data.get(guild_id).map(|gd| gd.run);
    field
}
