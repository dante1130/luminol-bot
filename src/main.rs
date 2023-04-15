use std::collections::HashMap;

use dotenv::dotenv;

use luminol_bot::commands::OpenAIClient;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let discord_token =
        std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&discord_token, intents)
        .event_handler(luminol_bot::handler::Handler)
        .framework(luminol_bot::commands::framework())
        .type_map_insert::<OpenAIClient>(HashMap::from([(0, async_openai::Client::new())]))
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
