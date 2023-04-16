use async_openai::types::{CreateImageRequestArgs, ImageSize, ResponseFormat};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::OpenAIClient;

#[command]
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

    msg.channel_id
        .send_files(&ctx.http, vec![paths[0].as_path()], |m| m)
        .await?;

    std::fs::remove_file(paths[0].as_path())?;

    Ok(())
}
