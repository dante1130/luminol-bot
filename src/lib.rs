pub mod commands;
pub mod handler;

use std::collections::{HashMap, VecDeque};

use async_openai::config::OpenAIConfig;
use commands::{
    audio::{join::join, leave::leave, play::play, skip::skip, stop::stop},
    general::{ask::ask, help::help, ping::ping, remind::remind},
    openai::{chat::chat, image::image, memorise::memorise, vision::vision},
};
use serenity::{all::CreateAttachment, futures::lock::Mutex};

pub struct Data {
    pub openai_client: async_openai::Client<OpenAIConfig>,
    pub ds_gif: CreateAttachment,
    pub memory_map: Mutex<HashMap<u64, VecDeque<String>>>,
    pub http_client: reqwest::Client,
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
                play(),
                join(),
                leave(),
                skip(),
                stop(),
                ping(),
                ask(),
                remind(),
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
                    http_client: reqwest::Client::new(),
                })
            })
        })
        .build();

    framework
}
