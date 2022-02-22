use serenity::client::bridge::gateway::ShardId;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::ShardManagerContainer;

#[command]
#[bucket = "meta"]
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
                "This Guilds' Shard ({}) latency is: {:?}",
                ctx.shard_id,
                runner.latency.unwrap()
            ),
        )
        .await?;

    Ok(())
}

#[command]
#[only_in("guilds")]
#[bucket = "meta"]
async fn serverinfo(ctx: &Context, message: &Message) -> CommandResult {
    let raw_guild_id = message.guild_id.unwrap();
    let raw_guild = raw_guild_id.to_guild_cached(ctx.cache.as_ref()).await;
    let guild = raw_guild.unwrap();

    let embed = Embed::fake(|e| {
        e.title(format!("{} | Server Info", guild.name))
            .field("ID", guild.id, true)
    });

    message.channel_id.say(&ctx.http, &embed).await?;

    Ok(())
}
