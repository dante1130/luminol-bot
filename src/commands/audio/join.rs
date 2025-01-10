use anyhow::anyhow;

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, category = "Audio")]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let (guild_id, voice_channel_id) = {
        let guild = match ctx.guild() {
            Some(guild) => guild,
            None => {
                return Err(anyhow!("This command can only be used in a server.").into());
            }
        };

        let voice_state = guild.voice_states.get(&ctx.author().id);

        let voice_channel_id = match voice_state {
            Some(state) => state.channel_id.unwrap(),
            None => {
                return Err(anyhow!("You are not in a voice channel.").into());
            }
        };

        (guild.id, voice_channel_id)
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird not initialized");

    manager.join(guild_id, voice_channel_id).await?;

    Ok(())
}
