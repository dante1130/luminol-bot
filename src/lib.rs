pub mod commands;
pub mod handler;

use commands::{
    general::{ask::ask, ping::ping},
    help::help,
};

pub struct Data {}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, hide_in_help)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

pub fn framework() -> poise::Framework<Data, Error> {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("e!".to_string()),
                ..Default::default()
            },
            commands: vec![ping(), ask(), register(), help()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    framework
}
