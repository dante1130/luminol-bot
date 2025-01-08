use std::collections::HashMap;

use async_openai::config::OpenAIConfig;
use serenity::{
    framework::{
        standard::{macros::group, Configuration},
        StandardFramework,
    },
    model::prelude::UserId,
    prelude::TypeMapKey,
};

pub mod commands;
pub mod games;
pub mod handler;

use commands::{
    general::ask::*, general::ping::*, help::*, openai::chat::*, openai::complete::*,
    openai::image::*,
};

pub struct OpenAIClient;

impl TypeMapKey for OpenAIClient {
    type Value = HashMap<u8, async_openai::Client<OpenAIConfig>>;
}

#[group]
#[commands(ping, ask)]
struct General;

#[group]
#[commands(complete, chat, image)]
struct OpenAI;

pub fn framework() -> StandardFramework {
    let framework = StandardFramework::new()
        .group(&GENERAL_GROUP)
        .group(&OPENAI_GROUP)
        .help(&HELP);

    framework.configure(Configuration::new().prefix("e!"));

    framework
}
