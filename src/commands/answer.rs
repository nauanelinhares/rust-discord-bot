use serenity::all::standard::Args;
use serenity::all::MessageBuilder;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::AIProviderContainer;

#[command]
async fn answer(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let question = args.rest();

    if question.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Please ask me a question!")
            .await?;
        return Ok(());
    }

    // Send typing indicator while waiting for AI response
    msg.channel_id.broadcast_typing(&ctx.http).await?;

    // Get AI provider from context data
    let data = ctx.data.read().await;
    let provider = data
        .get::<AIProviderContainer>()
        .expect("AI provider not initialized");

    let answer = match provider.generate(question).await {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("Error calling AI provider ({}): {}", provider.name(), e);
            format!(
                "Sorry, I couldn't get an answer from {}: {}",
                provider.name(),
                e
            )
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
