use async_openai::types::{CreateImageRequestArgs, ImageSize, ResponseFormat};
use serenity::{
    builder::{CreateAttachment, CreateMessage},
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::OpenAIClient;

#[command]
#[description("Generate an image with OpenAI's DALL·E API.")]
#[usage("<prompt>")]
pub async fn image(ctx: &Context, msg: &Message) -> CommandResult {
    let prompt = msg.content.trim_start_matches("e!image").trim();

    let request = CreateImageRequestArgs::default()
        .n(1)
        .prompt(prompt.to_owned())
        .response_format(ResponseFormat::Url)
        .size(ImageSize::S256x256)
        .user("async-openai")
        .build()
        .unwrap();

    let data = ctx.data.read().await;

    let client = data.get::<OpenAIClient>().unwrap().get(&0).unwrap();

    let response = client.images().create(request).await.unwrap();

    let paths = response.save("./res").await?;

    let attachment = CreateAttachment::path(paths[0].as_path()).await?;

    msg.channel_id
        .send_files(&ctx.http, [attachment], CreateMessage::default())
        .await?;

    std::fs::remove_file(paths[0].as_path())?;

    Ok(())
}
