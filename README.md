# luminol-bot

LuminolBot is a Discord bot that provides a variety of commands for the Luminol server. The previous bot was written in JavaScript and was stuck in dependency hell. This bot is written in Rust and uses the [serenity](https://github.com/serenity-rs/serenity) library.

The commands are invoked by prefixing the command with `e!`.

Here is a list of the commands:

- `ping` - Returns the message "Pong!". This is mostly to test the bot's latency.

## Table of Contents

- [luminol-bot](#luminol-bot)
  - [Table of Contents](#table-of-contents)
  - [Deploying the bot](#deploying-the-bot)
    - [Prerequisites](#prerequisites)
    - [Environment variables](#environment-variables)
    - [Building and running](#building-and-running)

## Deploying the bot

### Prerequisites

You will need to install `Rust` and `cargo`. You can do this by following the instructions [here](https://www.rust-lang.org/tools/install).

It is recommended that you use `rustup` to manage your Rust installation.

### Environment variables

You will need a `.env` file in the root directory of the project. This file should contain the following variables:

```bash
DISCORD_TOKEN=<your discord bot token>
```

### Building and running

```bash
cargo build --release
cargo run --release
```
