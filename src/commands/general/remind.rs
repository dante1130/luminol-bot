use serenity::all::Mentionable;

use crate::Context;
use crate::Error;

#[poise::command(slash_command, prefix_command, category = "General")]
pub async fn remind(
    ctx: Context<'_>,
    minutes: u64,
    #[rest] arg: Option<String>,
) -> Result<(), Error> {
    let reminder_message = arg.unwrap_or("Reminder!".to_string());

    ctx.reply(format!(
        "Reminder set for {} minutes from now: {}",
        minutes, reminder_message
    ))
    .await?;

    let author_mention = ctx.author().mention();
    let channel_id = ctx.channel_id();
    let http = ctx.serenity_context().http.clone();

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(minutes * 60)).await;

        let _ = channel_id
            .say(
                &http,
                format!("Reminder for {}: {}", author_mention, reminder_message),
            )
            .await;
    });

    Ok(())
}
