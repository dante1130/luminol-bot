use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::{games::bagels::*, Bagels};

#[command]
#[description(
    "Play a game of bagels!
     Pico = 1 digit correct.
     Fermi = 1 digit correct and in the right place.
     Bagels = no digits correct.
     You can specify the number of digits with `e!bagels new <number>`.
     The default and the minimum is 3 digits, and the maximum is 5 digits."
)]
#[usage("<number>")]
#[example("new")]
#[example("new 4")]
#[example("123")]
#[min_args(1)]
#[max_args(2)]
pub async fn bagels(ctx: &Context, msg: &Message) -> CommandResult {
    let args = msg.content.trim_start_matches("e!bagels").trim();
    let args_vec = args.split(' ').collect::<Vec<&str>>();

    let mut data = ctx.data.write().await;
    let bagels_game_map = data.get_mut::<Bagels>().unwrap().get_mut(&0).unwrap();

    if args_vec[0] == "new" {
        if bagels_game_map.contains_key(&msg.author.id) {
            msg.reply(&ctx.http, "You already have a game running!")
                .await?;
            return Ok(());
        }

        let digits = if args_vec.len() > 1 {
            match args_vec[1].parse::<usize>() {
                Ok(digits) => digits,
                Err(_) => 3,
            }
        } else {
            3
        };

        bagels_game_map.insert(msg.author.id, BagelsGameState::new(digits));
        msg.reply(
            &ctx.http,
            "Started a new game of bagels! Guess by using `e!bagels <number>`.",
        )
        .await?;

        return Ok(());
    }

    if bagels_game_map.contains_key(&msg.author.id) {
        let game = bagels_game_map.get_mut(&msg.author.id).unwrap();

        let guess_result = game.guess(args.to_owned());
        let result = match guess_result {
            Ok(result) => result,
            Err(err) => {
                msg.reply(&ctx.http, err).await?;
                return Ok(());
            }
        };

        match game.get_state() {
            BagelsState::Won => {
                msg.reply(
                    &ctx.http,
                    format!("You won! The number was {}.", game.get_secret()),
                )
                .await?;
                bagels_game_map.remove(&msg.author.id);
            }
            BagelsState::Lost => {
                msg.reply(
                    &ctx.http,
                    format!("You lost! The number was {}.", game.get_secret()),
                )
                .await?;
                bagels_game_map.remove(&msg.author.id);
            }
            _ => {
                msg.reply(&ctx.http, format!("{}", result)).await?;
            }
        }
    } else {
        msg.reply(
            &ctx.http,
            "You must start a game first! Use the command `e!bagels new`.",
        )
        .await?;
    }

    Ok(())
}
