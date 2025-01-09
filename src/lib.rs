pub mod commands;
pub mod handler;

use async_openai::config::OpenAIConfig;
use commands::{
    general::{ask::ask, ping::ping},
    help::help,
    openai::{chat::chat, image::image},
};
use serenity::all::CreateAttachment;

pub struct Data {
    pub openai_client: async_openai::Client<OpenAIConfig>,
    pub chat_prompt: String,
    pub ds_gif: CreateAttachment,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, hide_in_help)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

pub fn framework(
    openai_client: async_openai::Client<OpenAIConfig>,
) -> poise::Framework<Data, Error> {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("e!".to_string()),
                ..Default::default()
            },
            commands: vec![ping(), ask(), image(), chat(), register(), help()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                const INITIAL_PROMPT: &str = concat!(
                    "You are Ema Skye from the game Ace Attorney, you have an aspiration to be a forensic scientist\n",
                    "You are cheerful and optimistic, especially when it comes to forensic science.\n",
                    "Keep your sentences nice and brief, around 32 words.\n"
                );

                const MEMORY: &str = include_str!("../res/memory.txt");

                let chat_prompt = format!("{}{}", INITIAL_PROMPT, MEMORY);

                let ds_gif = CreateAttachment::bytes(include_bytes!("../res/DS.gif"), "DS.gif");

                Ok(Data {
                    openai_client,
                    chat_prompt,
                    ds_gif,
                })
            })
        })
        .build();

    framework
}
