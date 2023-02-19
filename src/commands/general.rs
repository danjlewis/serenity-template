use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::prelude::Message,
    prelude::*,
};
use tracing::{Instrument, Level};

#[group]
#[commands(ping, echo, greet)]
struct General;

#[command]
#[description("Ping pong!")]
#[num_args(0)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    async move {
        msg.reply(ctx, "Pong!").await?;

        Ok(())
    }
    .instrument(span!(
        Level::ERROR,
        "ping",
        author_id = u64::from(msg.author.id)
    ))
    .await
}

#[command]
#[description("Echoes your message back to you.")]
#[usage("<message>")]
async fn echo(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    async move {
        args.trimmed().quoted();

        let reply_content = args.remains().unwrap_or("*(silence)*");

        msg.reply(ctx, reply_content).await?;

        Ok(())
    }
    .instrument(span!(
        Level::ERROR,
        "echo",
        author_id = u64::from(msg.author.id)
    ))
    .await
}

#[command]
#[description("Says hello!")]
#[usage("[name='world']")]
#[example("Wumpus")]
#[max_args(1)]
async fn greet(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    async move {
        args.trimmed().quoted();

        let name = args.single::<String>().unwrap_or(String::from("world"));
        let reply_content = format!("Hello {name}!");

        msg.reply(ctx, reply_content).await?;

        Ok(())
    }
    .instrument(span!(
        Level::ERROR,
        "greet",
        author_id = u64::from(msg.author.id)
    ))
    .await
}
