use anyhow::anyhow;

use songbird::input::YoutubeDl;

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, category = "Audio")]
pub async fn play(ctx: Context<'_>, url: String) -> Result<(), Error> {
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird not initialized");

    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            return Err(anyhow!("Not in a guild to play.").into());
        }
    };

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            return Err(anyhow!("I'm not currently in a voice channel to play.").into());
        }
    };

    let mut handler = handler_lock.lock().await;

    let src = YoutubeDl::new(ctx.data().http_client.clone(), url.clone());

    handler.enqueue_input(src.into()).await;

    ctx.say(format!(
        "Queued {} at position {}.",
        url,
        handler.queue().len()
    ))
    .await?;

    Ok(())
}
