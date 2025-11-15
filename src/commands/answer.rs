use serenity::all::standard::Args;
use serenity::all::MessageBuilder;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::gemini;

#[command]
async fn answer(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let question = args.rest();

    if question.is_empty() {
        msg.channel_id.say(&ctx.http, "Please ask me a question!").await?;
        return Ok(());
    }

    // Send typing indicator while waiting for Gemini
    msg.channel_id.broadcast_typing(&ctx.http).await?;

    let answer = match gemini::ask_gemini(question).await {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("Error calling Gemini API: {}", e);
            format!("Sorry, I couldn't get an answer from Gemini: {}", e)
        }
    };

    let response = MessageBuilder::new()
        .push(&msg.author.name)
        .push(" Asked: ")
        .push(question)
        .push("\n\nAnswer: ")
        .push(answer)
        .build();

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}
