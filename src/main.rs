use serenity::async_trait;
use serenity::client::{Client, EventHandler};
use serenity::framework::standard::{StandardFramework, macros::{
    group,
    command,
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use std::ops::Add;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
}

#[group]
#[commands(echo)]
struct General;

async fn get_prefix(context: &Context, message: &Message) -> Option<String>{
    Some("td!".to_string())
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.dynamic_prefix(|context, message| Box::pin(
            get_prefix(context, message)
        ))).group(&GENERAL_GROUP);
    let token = std::env::var("TROPICAL_TOKEN").unwrap_or_else(|e| {
        eprintln!("Failed to get token, {}.", e);
        std::process::exit(1)
    });
    let mut client = Client::builder(token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to build client, {}", e);
            std::process::exit(1);
        });
    client.start().await.unwrap_or_else(
        |e| {
            eprintln!("Failed to start client, {}", e);
            std::process::exit(1);
        }
    );
}
#[command]
async fn echo(context: &Context, message: &Message) -> CommandResult {
    let prefix = get_prefix(&context, &message).await.unwrap_or_else(|| "".to_string()).add("echo");
    let content = &message.content.strip_prefix(&prefix).unwrap_or(&message.content);
    let channel = message.channel(&context.cache).await;
    match channel {
        Some(c) => {
            c.id().say(&context, content).await?;
            message.delete(context).await?;
        },
        _ => {
            println!("no channel")
        }
    }
    Ok(())
}