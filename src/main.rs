use std::collections::HashMap;

use shuttle_secrets::SecretStore;

use dotenv::dotenv;

use serenity::prelude::*;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    dotenv().ok();
    let discord_token =
        std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&discord_token, intents)
        .event_handler(luminol_bot::handler::Handler)
        .framework(luminol_bot::framework())
        .type_map_insert::<luminol_bot::OpenAIClient>(HashMap::from([(
            0,
            async_openai::Client::new(),
        )]))
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(client.into())
}
