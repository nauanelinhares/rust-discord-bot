use serenity::all::standard::Args;
use serenity::all::MessageBuilder;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn answer(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let question = args.rest();
    let answer = match question {
        "What is the meaning of life?" => "42",
        "Who is love of my life?" => "Kubra",
        "What is the airspeed velocity of an unladen swallow?" => "African or European?",
        _ => "I don't know the answer to that question.",
    };

    let response = MessageBuilder::new()
        .push(&msg.author.name)
        .push(" Asked: ")
        .push(question)
        .push("\nAnswer: ")
        .push(answer)
        .build();

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}
