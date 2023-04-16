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

    let question = msg.content.trim_start_matches("e!ask").trim();

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

    let response = responses.choose(&mut rand::thread_rng()).unwrap();

    msg.channel_id
        .send_files(&ctx.http, vec!["res/DS.gif"], |m| {
            m.embed(|e| {
                e.title(question);
                e.description(prepend_response.to_owned() + response);
                e.color(0xF6DBD8);
                e.attachment("DS.gif");
                e
            });
            m
        })
        .await?;

    Ok(())
}
