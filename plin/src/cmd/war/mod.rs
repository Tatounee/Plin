mod post;
mod run;
mod set;

use serenity::framework::standard::macros::group;

use post::POST_COMMAND;
use run::{START_COMMAND, STOP_COMMAND};
use set::WAR_SET_COMMAND;

#[group("Guerre")]
#[commands(war_set, post, start, stop)]
#[prefixes("guerre", "gr", "riverrace", "rr")]
pub struct War;
