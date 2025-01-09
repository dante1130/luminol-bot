use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs,
};

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, category = "OpenAI")]
pub async fn chat(ctx: Context<'_>, #[rest] arg: String) -> Result<(), Error> {
    const MAX_TOKENS: u16 = 64;

    let data = ctx.data();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .messages([
            {
                ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
                    content: ChatCompletionRequestSystemMessageContent::Text(
                        data.chat_prompt.clone(),
                    ),
                    name: None,
                })
            },
            {
                ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                    content: ChatCompletionRequestUserMessageContent::Text(arg),
                    name: Some(ctx.author().name.to_owned().replace(' ', "_")),
                })
            },
        ])
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
