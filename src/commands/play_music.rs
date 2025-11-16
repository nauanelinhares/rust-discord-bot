use serenity::all::standard::Args;
use serenity::all::{
    ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter, CreateMessage,
};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn play_music(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.rest();

    if query.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Por favor, forneça um termo de busca.")
            .await?;
        return Ok(());
    }

    let result = youtube_search::YoutubeSearch::search(query).await?;

    Ok(())
}
