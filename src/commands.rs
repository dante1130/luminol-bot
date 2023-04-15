use std::collections::HashSet;

use serenity::{
    framework::{
        standard::{
            help_commands,
            macros::{command, group, help},
            Args, CommandGroup, CommandResult, HelpOptions,
        },
        StandardFramework,
    },
    model::prelude::{Message, UserId},
    prelude::Context,
};

#[group]
#[commands(ping, ask)]
struct General;

pub fn framework() -> StandardFramework {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("e!"))
        .group(&GENERAL_GROUP)
        .help(&HELP);

    framework
}

#[help]
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

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong!").await?;

    Ok(())
}

#[command]
pub async fn ask(ctx: &Context, msg: &Message) -> CommandResult {
    let prepend_response = "So, to scientically analyze the data available so far, ";

    let question = msg.content.trim_start_matches("e!ask").trim();

    msg.channel_id
        .send_files(&ctx.http, vec!["res/DS.gif"], |m| {
            m.embed(|e| {
                e.title(question);
                e.description(prepend_response);
                e.color(0xF6DBD8);
                e.attachment("DS.gif");
                e
            });
            m
        })
        .await?;

    Ok(())
}
