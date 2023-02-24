#[macro_use]
extern crate tracing;

use std::env;

use anyhow::{Context, Result};
use serenity::prelude::*;
use tracing_subscriber::{filter::Directive, util::SubscriberInitExt};

mod commands;
mod handler;

pub const EMBED_COLOR: [u8; 3] = [0x58, 0x65, 0xF2]; // note: this value is mirrored in src/commands/help.rs

fn logger() -> impl SubscriberInitExt {
    use tracing_subscriber::EnvFilter;

    const CARGO_BIN_NAME: &str = env!("CARGO_BIN_NAME");
    const DEFAULT_FILTER_LEVEL: &str = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };

    let default_directive = format!("{CARGO_BIN_NAME}={DEFAULT_FILTER_LEVEL}")
        .parse::<Directive>()
        .expect("default directive should be valid");

    let env_filter = EnvFilter::builder()
        .with_default_directive(default_directive)
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .finish()
}

async fn client() -> Result<Client> {
    let token =
        env::var("DISCORD_TOKEN").context("Failed to load `DISCORD_TOKEN` environment variable")?;
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(token, intents)
        .event_handler(handler::Handler)
        .framework(commands::framework())
        .await
        .expect("client should build successfully");

    Ok(client)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    logger()
        .try_init()
        .expect("logger initialization shouldn't fail");

    let mut client = client().await.context("Failed to build client")?;
    client.start().await.context("Client error occurred")?;

    Ok(())
}
