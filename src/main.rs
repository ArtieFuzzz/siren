extern crate dotenv;

mod commands;

use commands::meta::*;
use std::{collections::HashSet, env, sync::Arc};

use serenity::{
    async_trait,
    client::bridge::gateway::{GatewayIntents, ShardManager},
    framework::{
        standard::{
            macros::{group, hook},
            DispatchError,
        },
        StandardFramework,
    },
    http::Http,
    model::{channel::Message, gateway::Activity, gateway::Ready},
    prelude::*,
};
use tracing::info;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Logged in as {}!", ready.user.tag());
        ctx.set_activity(Activity::watching("The stars above"))
            .await;
    }
}

#[group]
#[commands(ping)]
struct General;

#[hook]
async fn delay_action(ctx: &Context, message: &Message) {
    let _ = message.react(ctx, '⏱').await;
}

#[hook]
async fn dispatch_error(ctx: &Context, message: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(ref info) = error {
        // We notify them only once.
        if info.is_first_try {
            let _ = message
                .channel_id
                .say(
                    &ctx.http,
                    &format!("Try this again in {} seconds.", info.as_secs()),
                )
                .await;
        }
    }

    if let DispatchError::NotEnoughArguments { min, given } = error {
        let _ = message
            .channel_id
            .say(
                &ctx.http,
                format!("{} Arguments must be given, {} was given", min, given),
            )
            .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Could not load .env");

    tracing_subscriber::fmt::init();

    let token = env::var("TOKEN").expect("TOKEN is not set in .env");
    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!!"))
        .group(&GENERAL_GROUP)
        .bucket("meta", |b| b.delay(2))
        .await
        .bucket("complicated", |b| {
            b.limit(1).time_span(30).delay_action(delay_action)
        })
        .await;

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .intents(GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES)
        .await
        .expect("Client went brrr");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register Termination handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        println!("Client Err: {:?}", why);
    }
}
