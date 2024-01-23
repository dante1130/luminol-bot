use rand::seq::SliceRandom;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
#[description("Ask Ema a question and she will answer it!")]
#[usage("<question>")]
pub async fn ask(ctx: &Context, msg: &Message) -> CommandResult {
    let prepend_response = "So, to scientically analyze the data available so far, ";

    let responses = vec![
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

    let response = responses
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();

    msg.channel_id
        .say(&ctx.http, format!("{} {}", prepend_response, response))
        .await?;

    Ok(())
}
