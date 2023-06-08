use std::collections::HashMap;

use serenity::{
    framework::{standard::macros::group, StandardFramework},
    model::prelude::UserId,
    prelude::TypeMapKey,
};

pub mod commands;
pub mod games;
pub mod handler;

use commands::{
    games::bagels::*, general::ask::*, general::ping::*, help::*, openai::chat::*,
    openai::complete::*, openai::image::*,
};

pub struct OpenAIClient;

impl TypeMapKey for OpenAIClient {
    type Value = HashMap<u8, async_openai::Client>;
}

pub struct Bagels;

impl TypeMapKey for Bagels {
    type Value = HashMap<u8, HashMap<UserId, games::bagels::BagelsGameState>>;
}

#[group]
#[commands(ping, ask)]
struct General;

#[group]
#[commands(bagels)]
struct Games;

#[group]
#[commands(complete, chat, image)]
struct OpenAI;

pub fn framework() -> StandardFramework {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("e!"))
        .group(&GENERAL_GROUP)
        .group(&GAMES_GROUP)
        .group(&OPENAI_GROUP)
        .help(&HELP);

    framework
}
