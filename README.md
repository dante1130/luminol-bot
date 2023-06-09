# luminol-bot

LuminolBot is a Discord bot that provides a variety of features such as asking the bot any questions you want, games and making calls to the OpenAI API. The previous bot was written in JavaScript and was stuck in dependency hell. This bot is written in Rust and uses the [serenity](https://github.com/serenity-rs/serenity) library.

The commands are invoked by prefixing the command with `e!`. You can see the list of commands by typing `e!help`.

## Table of Contents

- [luminol-bot](#luminol-bot)
  - [Table of Contents](#table-of-contents)
  - [Deploying the bot](#deploying-the-bot)
    - [Prerequisites](#prerequisites)
    - [Secrets](#secrets)
    - [Building and running](#building-and-running)
    - [Deploying to Shuttle](#deploying-to-shuttle)

## Deploying the bot

### Prerequisites

You will need to install `Rust` and `cargo`. You can do this by following the instructions [here](https://www.rust-lang.org/tools/install).

It is recommended that you use `rustup` to manage your Rust installation.

### Secrets

You will need a `Secrets.toml` file in the root directory of the project. This file should contain the following variables:

```env
DISCORD_TOKEN=<your discord bot token>
OPENAI_API_KEY=<your openai api key>
```

### Building and running

```bash
cargo shuttle run
```

### Deploying to Shuttle

```bash
cargo shuttle deploy
```
