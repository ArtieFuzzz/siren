use serenity::{
    async_trait,
    model::{gateway::Activity, gateway::Ready},
    prelude::*,
};
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Logged in as {}!", ready.user.tag());
        ctx.set_activity(Activity::watching("The stars above"))
            .await;

        if let Some(shard) = ready.shard {
            info!(
                "{} is connected on Shard {}/{}",
                ready.user.tag(),
                shard[0] + 1,
                shard[1]
            );
        }
    }
}
