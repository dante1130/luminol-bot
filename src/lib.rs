use std::collections::HashMap;

use serenity::{
    framework::{standard::macros::group, StandardFramework},
    prelude::TypeMapKey,
};

pub mod commands;
pub mod handler;

use commands::{general::ask::*, general::ping::*, help::*, openai::chat::*, openai::complete::*};

pub struct OpenAIClient;

impl TypeMapKey for OpenAIClient {
    type Value = HashMap<u8, async_openai::Client>;
}

#[group]
#[commands(ping, ask)]
struct General;

#[group]
#[commands(complete, chat)]
struct OpenAI;

pub fn framework() -> StandardFramework {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("e!"))
        .group(&GENERAL_GROUP)
        .group(&OPENAI_GROUP)
        .help(&HELP);

    framework
}
