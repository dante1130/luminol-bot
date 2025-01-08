use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, category = "General")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await;
    ctx.say(format!("Pong! Latency is {}ms", latency.as_millis()))
        .await?;
    Ok(())
}
