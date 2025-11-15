use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!(
        "Ping command used by user '{}' in channel {}",
        msg.author.name, msg.channel_id
    );

    match msg.channel_id.say(&ctx.http, "Pong!").await {
        Ok(_) => {
            println!("Successfully sent Pong! response");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error sending message: {:?}", e);
            Err(e.into())
        }
    }
}
