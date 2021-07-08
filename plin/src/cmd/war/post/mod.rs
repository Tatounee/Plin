mod new;
mod update;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use new::NEW_POST_COMMAND;
use update::UPDATE_POST_COMMAND;

#[command("post")]
#[num_args(0)]
#[sub_commands(update_post, new_post)]
#[description("Outils relatifs aux posts.")]
#[required_permissions("ADMINISTRATOR")]
async fn post(_: &Context, _: &Message, _: Args) -> CommandResult {
    Ok(())
}
