use crate::Context;
use crate::Error;

#[poise::command(slash_command, prefix_command)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "Type `e!help command` for more info on a command",
        ..Default::default()
    };

    poise::builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}
