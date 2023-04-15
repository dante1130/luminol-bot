use std::collections::{HashMap, HashSet};

use serenity::{
    framework::{
        standard::{
            help_commands,
            macros::{command, group, help},
            Args, CommandGroup, CommandResult, HelpOptions,
        },
        StandardFramework,
    },
    model::prelude::{Message, UserId},
    prelude::{Context, TypeMapKey},
};

use async_openai::types::{
    ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, CreateCompletionRequestArgs,
    Role,
};

use rand::seq::SliceRandom;

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

#[help]
pub async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong!").await?;

    Ok(())
}

#[command]
pub async fn ask(ctx: &Context, msg: &Message) -> CommandResult {
    let prepend_response = "So, to scientically analyze the data available so far, ";

    let question = msg.content.trim_start_matches("e!ask").trim();

    let responses = vec![
        "as I see it, yes.",
        "ask again later.",
        "better not tell you now.",
        "cannot predict now.",
        "concentrate and ask again.",
        "don't count on it.",
        "it is certain.",
        "it is decidedly so.",
        "most likely.",
        "my reply is no.",
        "my sources say no.",
        "outlook not so good.",
        "outlook good.",
        "reply hazy, try again.",
        "signs point to yes.",
        "very doubtful.",
        "without a doubt.",
        "yes.",
        "yes - definitely.",
        "you may rely on it.",
    ];

    let response = responses.choose(&mut rand::thread_rng()).unwrap();

    msg.channel_id
        .send_files(&ctx.http, vec!["res/DS.gif"], |m| {
            m.embed(|e| {
                e.title(question);
                e.description(prepend_response.to_owned() + response);
                e.color(0xF6DBD8);
                e.attachment("DS.gif");
                e
            });
            m
        })
        .await?;

    Ok(())
}

pub struct OpenAIClient;

impl TypeMapKey for OpenAIClient {
    type Value = HashMap<u8, async_openai::Client>;
}

#[command]
pub async fn complete(ctx: &Context, msg: &Message) -> CommandResult {
    let prompt = msg.content.trim_start_matches("e!complete").trim();

    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(prompt)
        .max_tokens(128_u16)
        .build()
        .unwrap();

    let data = ctx.data.read().await;

    let client = data.get::<OpenAIClient>().unwrap().get(&0).unwrap();

    let response = client.completions().create(request).await.unwrap();

    msg.channel_id
        .say(&ctx.http, &response.choices.first().unwrap().text)
        .await?;

    Ok(())
}

#[command]
pub async fn chat(ctx: &Context, msg: &Message) -> CommandResult {
    let prompt = msg.content.trim_start_matches("e!chat").trim();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages(vec![
            {
                ChatCompletionRequestMessage {
                    role: Role::System,
                    content: "You are Ema Skye from the game Ace Attorney, you have an aspiration to be a forensic scientist. 
                              You are cheerful and optimistic, especially when it comes to forensic science.".to_owned(),
                    name: None,
                }
            },
            {
                ChatCompletionRequestMessage {
                    role: Role::Assistant,
                    content: "Ask away! With the power of science, 
                              I'll scientifically analyze the data available
                              and use my scientific gadgets to solve your problems.".to_owned(),
                    name: Some("EmaSkye".to_owned()),
                }
            },
            {
                ChatCompletionRequestMessage {
                    role: Role::User,
                    content: prompt.to_owned(),
                    name: Some(msg.author.name.to_owned()),
                }
            },
        ])
        .max_tokens(128_u16)
        .build()
        .unwrap();

    let data = ctx.data.read().await;

    let client = data.get::<OpenAIClient>().unwrap().get(&0).unwrap();

    let response = client.chat().create(request).await.unwrap();

    msg.channel_id
        .say(
            &ctx.http,
            &response.choices.first().unwrap().message.content,
        )
        .await?;

    Ok(())
}
