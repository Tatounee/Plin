mod post;
mod reset;
mod run;
mod set;

use serenity::framework::standard::macros::group;

use post::POST_COMMAND;
use reset::WAR_RESET_COMMAND;
use run::{START_COMMAND, STOP_COMMAND};
use set::WAR_SET_COMMAND;

#[group("Guerre")]
#[commands(war_set, war_reset, post, start, stop)]
#[prefixes("guerre", "gr", "riverrace", "rr")]
pub struct War;
