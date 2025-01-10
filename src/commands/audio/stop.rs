use anyhow::anyhow;

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, category = "Audio")]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird not initialized");

    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            return Err(anyhow!("Not in a guild.").into());
        }
    };

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            return Err(anyhow!("I'm not currently playing in a voice channel.").into());
        }
    };

    handler_lock.lock().await.queue().stop();

    ctx.say("Stopped and cleared queue.").await?;

    Ok(())
}
