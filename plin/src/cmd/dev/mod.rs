mod info;
mod setup;

use serenity::framework::standard::macros::group;

use info::INFO_COMMAND;
use setup::SETUP_COMMAND;

#[group]
#[commands(info, setup)]
#[required_permissions("ADMINISTRATOR")]
#[prefixes("dev")]
pub struct Dev;
