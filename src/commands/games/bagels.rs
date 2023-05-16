use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::{games::bagels::*, Bagels};

#[command]
#[description("Play a game of bagels!")]
#[usage("new")]
#[usage("<guess>")]
pub async fn bagels(ctx: &Context, msg: &Message) -> CommandResult {
    let args = msg.content.trim_start_matches("e!bagels").trim();

    let mut data = ctx.data.write().await;
    let bagels_game_map = data.get_mut::<Bagels>().unwrap().get_mut(&0).unwrap();

    if args == "new" {
        bagels_game_map.insert(msg.author.id, BagelsGameState::new(3));
        msg.reply(
            &ctx.http,
            "Started a new game of bagels! Guess by using `e!bagels <number>`.",
        )
        .await?;
    } else if bagels_game_map.contains_key(&msg.author.id) {
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
        msg.reply(&ctx.http, "You must start a game first!").await?;
    }

    Ok(())
}
