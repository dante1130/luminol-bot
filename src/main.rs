use anyhow::anyhow;

use async_openai::config::OpenAIConfig;
use shuttle_runtime::SecretStore;

use serenity::prelude::*;
use songbird::SerenityInit;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let discord_token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let openai_api_key = if let Some(token) = secret_store.get("OPENAI_API_KEY") {
        token
    } else {
        return Err(anyhow!("'OPENAI_API_KEY' was not found").into());
    };

    let openai_config = OpenAIConfig::new().with_api_key(openai_api_key);

    let openai_client = async_openai::Client::with_config(openai_config);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_VOICE_STATES;

    let client = serenity::client::Client::builder(&discord_token, intents)
        .event_handler(luminol_bot::handler::Handler)
        .framework(luminol_bot::framework(openai_client))
        .register_songbird()
        .await
        .expect("Err creating client");

    Ok(client.into())
}
