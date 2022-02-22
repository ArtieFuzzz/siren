extern crate dotenv;

use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.content == "!ping" {
            if let Err(why) = message.channel_id.say(&ctx.http, "Pong!").await {
                println!(
                    "There was an error while running the ping command {:?}",
                    why
                )
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN").expect("TOKEN is not set in .env");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Client went brrr");

    if let Err(why) = client.start().await {
        println!("Client Err: {:?}", why);
    }
}
