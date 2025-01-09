use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs,
};

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, category = "OpenAI")]
pub async fn chat(ctx: Context<'_>, #[rest] arg: String) -> Result<(), Error> {
    const MAX_TOKENS: u16 = 64;
    const CHAT_PROMPT: &'static str = include_str!("../../../res/memory.txt");

    let data = ctx.data();

    let memory_map = data.memory_map.lock().await;

    let prompt = if let Some(memory) = memory_map.get(&ctx.channel_id().get()) {
        format!(
            "{}{}",
            CHAT_PROMPT,
            memory.iter().cloned().collect::<Vec<String>>().join("\n")
        )
    } else {
        CHAT_PROMPT.to_string()
    };

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .messages([
            {
                ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
                    content: ChatCompletionRequestSystemMessageContent::Text(prompt),
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
