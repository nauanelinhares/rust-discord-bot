use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!("Ping command used by user '{}'", msg.author.name);
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}
