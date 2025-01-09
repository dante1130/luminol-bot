pub mod commands;
pub mod handler;

use std::collections::{HashMap, VecDeque};

use async_openai::config::OpenAIConfig;
use commands::{
    general::{ask::ask, ping::ping},
    help::help,
    openai::{chat::chat, image::image, memorise::memorise, vision::vision},
};
use serenity::{all::CreateAttachment, futures::lock::Mutex};

pub struct Data {
    pub openai_client: async_openai::Client<OpenAIConfig>,
    pub ds_gif: CreateAttachment,
    pub memory_map: Mutex<HashMap<u64, VecDeque<String>>>,
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
            commands: vec![
                ping(),
                ask(),
                image(),
                chat(),
                vision(),
                memorise(),
                register(),
                help(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data {
                    openai_client,
                    ds_gif: CreateAttachment::bytes(include_bytes!("../res/DS.gif"), "DS.gif"),
                    memory_map: HashMap::new().into(),
                })
            })
        })
        .build();

    framework
}
