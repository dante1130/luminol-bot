use anyhow::anyhow;

use shuttle_runtime::SecretStore;

use serenity::prelude::*;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let discord_token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::client::Client::builder(&discord_token, intents)
        .event_handler(luminol_bot::handler::Handler)
        .framework(luminol_bot::framework())
        .await
        .expect("Err creating client");

    Ok(client.into())
}
