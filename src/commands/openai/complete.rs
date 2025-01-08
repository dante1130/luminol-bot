use async_openai::types::CreateCompletionRequestArgs;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::OpenAIClient;

#[command]
#[description("Complete a prompt with OpenAI's GPT-3 API.")]
#[usage("<prompt>")]
pub async fn complete(ctx: &Context, msg: &Message) -> CommandResult {
    let prompt = msg.content.trim_start_matches("e!complete").trim();

    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(prompt)
        .max_tokens(128_u16)
        .build()
        .unwrap();

    let data = ctx.data.read().await;

    let client = data.get::<OpenAIClient>().unwrap().get(&0).unwrap();

    let response = client.completions().create(request).await.unwrap();

    msg.channel_id
        .say(&ctx.http, &response.choices.first().unwrap().text)
        .await?;

    Ok(())
}
