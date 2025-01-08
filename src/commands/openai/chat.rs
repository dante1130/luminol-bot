use async_openai::types::{
    ChatCompletionRequestAssistantMessage, ChatCompletionRequestAssistantMessageContent,
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs,
};

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, category = "OpenAI")]
pub async fn chat(ctx: Context<'_>, #[rest] arg: String) -> Result<(), Error> {
    const MAX_WORD_COUNT: u16 = 32;
    const MAX_TOKENS: u16 = 64;

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o")
        .messages([
            {
                ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
                    content: ChatCompletionRequestSystemMessageContent::Text(format!("You are Ema Skye from the game Ace Attorney, you have an aspiration to be a forensic scientist. 
                              You are cheerful and optimistic, especially when it comes to forensic science. Keep your sentences nice and brief, around {} words.", MAX_WORD_COUNT)),
                    name: None,
                })
            },
            {
                ChatCompletionRequestMessage::Assistant(ChatCompletionRequestAssistantMessage {
                    content: Some(ChatCompletionRequestAssistantMessageContent::Text("Ask away! With the power of science, 
                              I'll scientifically analyze the data available
                              and use my scientific gadgets to solve your problems.".to_owned())),
                    name: Some("Ema_Skye".to_owned()),
                    ..Default::default()
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

    let data = ctx.data();

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
