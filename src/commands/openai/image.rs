use async_openai::types::{CreateImageRequestArgs, ImageModel, ImageResponseFormat, ImageSize};

use serenity::builder::{CreateAttachment, CreateMessage};

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, category = "OpenAI")]
pub async fn image(ctx: Context<'_>, #[rest] arg: String) -> Result<(), Error> {
    let request = CreateImageRequestArgs::default()
        .model(ImageModel::DallE2)
        .n(1)
        .prompt(arg)
        .response_format(ImageResponseFormat::Url)
        .size(ImageSize::S256x256)
        .user("async-openai")
        .build()?;

    let data = ctx.data();

    let response = data.openai_client.images().create(request).await?;

    let paths = response.save("./res").await?;

    let attachment = CreateAttachment::path(paths[0].as_path()).await?;

    ctx.channel_id()
        .send_files(&ctx.http(), [attachment], CreateMessage::default())
        .await?;

    std::fs::remove_file(paths[0].as_path())?;

    Ok(())
}
