use serenity::client::bridge::gateway::ShardId;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::ShardManagerContainer;

#[command]
async fn ping(ctx: &Context, message: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(m) => m,
        None => {
            message
                .reply(ctx, "There was a problem getting the Shard Manager")
                .await?;

            return Ok(());
        }
    };

    let managers = shard_manager.lock().await;
    let runners = managers.runners.lock().await;

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            message.reply(ctx, "Shard not found...").await?;

            return Ok(());
        }
    };

    message
        .channel_id
        .say(
            &ctx.http,
            &format!(
                "This Guilds' Shard latency is: {:?}",
                runner.latency.unwrap()
            ),
        )
        .await?;

    Ok(())
}
