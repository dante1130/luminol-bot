use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestMessageContentPartImage,
    ChatCompletionRequestMessageContentPartText, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, ChatCompletionRequestUserMessageContentPart,
    CreateChatCompletionRequestArgs, ImageUrl,
};
use serenity::all::Attachment;

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, category = "OpenAI")]
pub async fn vision(
    ctx: Context<'_>,
    attachment: Attachment,
    #[rest] arg: String,
) -> Result<(), Error> {
    const MAX_TOKENS: u16 = 128;

    let data = ctx.data();

    let text_prompt = ChatCompletionRequestMessageContentPartText { text: arg };

    let image = ChatCompletionRequestMessageContentPartImage {
        image_url: ImageUrl {
            url: attachment.url.to_owned(),
            ..Default::default()
        },
    };

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .messages([{
            ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                content: ChatCompletionRequestUserMessageContent::Array(vec![
                    ChatCompletionRequestUserMessageContentPart::Text(text_prompt),
                    ChatCompletionRequestUserMessageContentPart::ImageUrl(image),
                ]),
                name: Some(ctx.author().name.to_owned().replace(' ', "_")),
            })
        }])
        .max_tokens(MAX_TOKENS)
        .n(1)
        .build()?;

    let response = data.openai_client.chat().create(request).await?;

    ctx.channel_id()
        .say(
            &ctx.http(),
            &response
                .choices
                .first()
                .unwrap()
                .message
                .content
                .clone()
                .unwrap(),
        )
        .await?;

    Ok(())
}
