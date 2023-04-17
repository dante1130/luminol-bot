use std::collections::HashMap;

use anyhow::anyhow;

use shuttle_secrets::SecretStore;

use serenity::prelude::*;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
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

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&discord_token, intents)
        .event_handler(luminol_bot::handler::Handler)
        .framework(luminol_bot::framework())
        .type_map_insert::<luminol_bot::OpenAIClient>(HashMap::from([(
            0,
            async_openai::Client::new().with_api_key(openai_api_key),
        )]))
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(client.into())
}
