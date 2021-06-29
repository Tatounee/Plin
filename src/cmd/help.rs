use std::collections::HashSet;

use serenity::{client::Context, framework::standard::{
        help_commands,
        macros::help,
        Args, CommandGroup, CommandResult, HelpOptions,
    }, model::{channel::Message, id::UserId}};

#[help]
#[no_help_available_text("Commande inconnue")]
#[usage_label = "Utilisation"]
#[usage_sample_label = "Exemple"]
#[ungrouped_label = "Sans groupe"]
#[grouped_label = "Groupe"]
#[sub_commands_label = "Commandes associées"]
#[description_label = "Description"]
#[available_text = "Disponibilité"]
#[aliases_label= "Alias"]
#[checks_label = "Checks"]
#[guild_only_text = "Sur un serveur seulement"]
#[dm_only_text = "En DM seulement"]
#[dm_and_guild_text = "En DM ou sur un serveur"]
#[command_not_found_text = "Command inconnu"]
#[suggestion_text = "Essayez `!plin help <cmd>`"]
#[individual_command_tip = ""]
#[strikethrough_commands_tip_in_guild = ""]
#[strikethrough_commands_tip_in_dm = ""]
#[group_prefix = "prefix"]
#[embed_success_colour("#FF8800")]
#[max_levenshtein_distance(3)]
#[wrong_channel = "hide"]
#[lacking_conditions = "hide"]
#[lacking_ownership = "hide"]
#[lacking_permissions = "hide"]
async fn plin_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}
