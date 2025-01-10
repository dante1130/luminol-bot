use anyhow::anyhow;

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, category = "Audio")]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            return Err(anyhow!("Not in a guild to leave.").into());
        }
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird not initialized");

    manager.leave(guild_id).await?;

    Ok(())
}
