extern crate dotenv;

mod commands;
mod handler;
mod hooks;

use commands::meta::*;
use handler::Handler;
use hooks::user_error;
use std::{collections::HashSet, env, sync::Arc};

use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    prelude::*,
};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[group]
#[commands(ping, serverinfo)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let token = env::var("TOKEN").expect("TOKEN is not set in .env");
    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!"))
        .group(&GENERAL_GROUP)
        .bucket("meta", |b| b.limit(1).delay(2))
        .await
        .bucket("complicated", |b| {
            b.limit(1)
                .time_span(30)
                .delay_action(user_error::delay_action)
        })
        .await;

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
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

    let shards = env::var("SHARDS")
        .unwrap_or(String::from("1"))
        .parse::<u64>()
        .unwrap();

    if let Err(why) = client.start_shards(shards).await {
        println!("Client Err: {:?}", why);
    }
}
