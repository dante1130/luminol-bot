use std::collections::VecDeque;

use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs,
};

use serenity::all::GetMessages;

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, category = "OpenAI")]
pub async fn memorise(ctx: Context<'_>, messages_to_memorise_count: Option<u8>) -> Result<(), Error> {
    const MAX_TOKENS: u16 = 64;
    const MAX_MEMORY_SIZE: usize = 50;
    const CHAT_PROMPT: &'static str = 
    "Please summarize the conversation in one sentence so that it can be used as a memory for the next conversation.";

    let messages = ctx.channel_id().messages(&ctx.http(), GetMessages::new().limit(messages_to_memorise_count.unwrap_or(50))).await?;
    
    let formatted_messages = messages.iter().map(|message| {
        format!("{}: {}", message.author.name, message.content)
    }).collect::<Vec<String>>().join("\n");

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .messages([
            {
                ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
                    content: ChatCompletionRequestSystemMessageContent::Text(
                        CHAT_PROMPT.to_string(),
                    ),
                    name: None,
                })
            },
            {
                ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                    content: ChatCompletionRequestUserMessageContent::Text(formatted_messages),
                    name: None,
                })
            },
        ])
        .max_tokens(MAX_TOKENS)
        .n(1)
        .build()?;

    let data = ctx.data();

    let response = data.openai_client.chat().create(request).await?;

    let response_content = response.choices.first().unwrap().message.content.clone().unwrap();

    let mut memory_map = data.memory_map.lock().await;

    if memory_map.contains_key(&ctx.channel_id().get()) {
        let memory = memory_map.get_mut(&ctx.channel_id().get()).unwrap();
        memory.push_back(response_content.clone());

        if memory.len() > MAX_MEMORY_SIZE {
            memory.pop_front();
        }
    } else {
        memory_map.insert(ctx.channel_id().get(), VecDeque::from([response_content.clone()]));

    }

    drop(memory_map);

    ctx.channel_id()
        .say(
            &ctx.http(),
            format!("Added the following to the memory:\n{}", response_content),
        )
        .await?;

    Ok(())
}
