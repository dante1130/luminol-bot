use rand::seq::SliceRandom;

use serenity::builder::{CreateAttachment, CreateEmbed, CreateMessage};

use crate::Context;
use crate::Error;

#[poise::command(slash_command, prefix_command, category = "General")]
pub async fn ask(ctx: Context<'_>, #[rest] arg: String) -> Result<(), Error> {
    const PREPEND_RESPONSE: &str = "So, to scientically analyze the data available so far, ";

    const RESPONSES: [&str; 20] = [
        "as I see it, yes.",
        "ask again later.",
        "better not tell you now.",
        "cannot predict now.",
        "concentrate and ask again.",
        "don't count on it.",
        "it is certain.",
        "it is decidedly so.",
        "most likely.",
        "my reply is no.",
        "my sources say no.",
        "outlook not so good.",
        "outlook good.",
        "reply hazy, try again.",
        "signs point to yes.",
        "very doubtful.",
        "without a doubt.",
        "yes.",
        "yes - definitely.",
        "you may rely on it.",
    ];

    let response = RESPONSES
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();

    let attachment = CreateAttachment::path("res/DS.gif").await?;

    let embed = CreateEmbed::default()
        .title(arg)
        .description(format!("{} {}", PREPEND_RESPONSE, response))
        .color(0xF6DBD8)
        .attachment(&attachment.filename);

    let message = CreateMessage::default().embed(embed);

    ctx.channel_id()
        .send_files(&ctx.http(), [attachment], message)
        .await?;

    Ok(())
}
