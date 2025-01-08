use poise::{samples::HelpConfiguration, serenity_prelude};

#[poise::command(slash_command)]
pub async fn help(context: poise::Context<'_>) -> anyhow::Result<()> {
    Ok(())
}
