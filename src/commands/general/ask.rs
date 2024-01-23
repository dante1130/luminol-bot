use rand::seq::SliceRandom;
use serenity::{
    builder::{CreateAttachment, CreateEmbed, CreateMessage},
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
#[description("Ask Ema a question and she will answer it!")]
#[usage("<question>")]
pub async fn ask(ctx: &Context, msg: &Message) -> CommandResult {
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

    let question = msg.content.trim_start_matches("e!ask").trim();

    let response = RESPONSES
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();

    let attachment = CreateAttachment::path("res/DS.gif").await?;

    let embed = CreateEmbed::default()
        .title(question)
        .description(format!("{} {}", PREPEND_RESPONSE, response))
        .color(0xF6DBD8)
        .attachment(&attachment.filename);

    let message = CreateMessage::default().embed(embed);

    msg.channel_id
        .send_files(&ctx.http, [attachment], message)
        .await?;

    Ok(())
}
