use async_openai::types::{
    ChatCompletionRequestAssistantMessage, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs, Role,
};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::OpenAIClient;

#[command]
#[description("Chat with OpenAI's GPT-3.5 API.")]
#[usage("<prompt>")]
pub async fn chat(ctx: &Context, msg: &Message) -> CommandResult {
    let prompt = msg.content.trim_start_matches("e!chat").trim();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages(vec![
            {
                ChatCompletionRequestMessage::System(
                    ChatCompletionRequestSystemMessage {
                    role: Role::System,
                    content: "You are Ema Skye from the game Ace Attorney, you have an aspiration to be a forensic scientist. 
                              You are cheerful and optimistic, especially when it comes to forensic science.".to_owned(),
                    name: None,
                })
            },
            {
                ChatCompletionRequestMessage::Assistant(ChatCompletionRequestAssistantMessage{
                    role: Role::Assistant,
                    content: Some("Ask away! With the power of science, 
                              I'll scientifically analyze the data available
                              and use my scientific gadgets to solve your problems.".to_owned()),
                    name: Some("Ema_Skye".to_owned()),
                    tool_calls: None,
                    function_call: None,
                })
            },
            {
                ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                    role: Role::User,
                    content: ChatCompletionRequestUserMessageContent::Text(prompt.to_owned()),
                    name: Some(msg.author.name.to_owned().replace(' ', "_")),
                })
            },
        ])
        .max_tokens(128_u16)
        .build()
        .unwrap();

    let data = ctx.data.read().await;

    let client = data.get::<OpenAIClient>().unwrap().get(&0).unwrap();

    let response = client.chat().create(request).await.unwrap();

    msg.channel_id
        .say(
            &ctx.http,
            &response
                .choices
                .first()
                .unwrap()
                .message
                .content
                .as_ref()
                .unwrap()
                .to_string(),
        )
        .await?;

    Ok(())
}
