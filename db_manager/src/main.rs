use std::error::Error;

use plin_data::{Id, PartialGuildData, DATABASE_PATH};

fn main() -> Result<(), Box<dyn Error>> {
    let db = sled::open(DATABASE_PATH)?;

    for item in db.iter() {
        let (key, value) = item?;
        let id: Id = key.into();
        let guild_data: PartialGuildData = value.into();

        println!("{:?}\n{:?}\n{:-<40}", id, guild_data, "");
    }

    Ok(())
}
