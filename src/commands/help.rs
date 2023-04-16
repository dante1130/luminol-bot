use std::collections::HashSet;

use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::prelude::{Message, UserId},
    prelude::Context,
};

#[help]
#[individual_command_tip = "Hello! I'm Ema, the Luminol bot. 
                            You can use `e!help` to get a list of all commands, 
                            or `e!help <command>` to get more information about a specific command."]
#[command_not_found_text = "Could not find: `{}`."]
pub async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
