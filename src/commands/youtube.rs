use serenity::all::standard::Args;
use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::YouTubeClientContainer;

#[command]
#[aliases("yt", "search")]
async fn youtube(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.rest();

    if query.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Por favor, forneça um termo de busca! Exemplo: !youtube rust tutorial")
            .await?;
        return Ok(());
    }

    // Send typing indicator while searching
    msg.channel_id.broadcast_typing(&ctx.http).await?;

    // Get YouTube client from context data
    let data = ctx.data.read().await;
    let youtube_client = data
        .get::<YouTubeClientContainer>()
        .expect("YouTube client not initialized");

    // Search for the first video
    match youtube_client.search_first_video(query).await {
        Ok((video_id, snippet)) => {
            let video_url = crate::youtube::YouTubeClient::get_video_url(&video_id);

            // Truncate description if too long
            let description = if snippet.description.len() > 200 {
                format!("{}...", &snippet.description[..200])
            } else {
                snippet.description.clone()
            };

            // Create a beautiful embed response
            let embed = CreateEmbed::new()
                .title(format!("🎥 {}", snippet.title))
                .url(&video_url)
                .description(&description)
                .color(0xFF0000) // YouTube red color
                .field("📺 Canal", &snippet.channel_title, true)
                .field("📅 Publicado", snippet.published_at.split('T').next().unwrap_or(&snippet.published_at), true)
                .field("🔍 Busca", query, false)
                .footer(CreateEmbedFooter::new("Use !youtube <termo> para buscar mais vídeos"))
                .timestamp(Timestamp::now());

            // Send the embed with the video URL
            msg.channel_id
                .send_message(
                    &ctx.http,
                    CreateMessage::new()
                        .content(&video_url)
                        .embed(embed)
                )
                .await?;
        }
        Err(e) => {
            let error_embed = CreateEmbed::new()
                .title("❌ Erro na Busca")
                .description(format!("Não foi possível buscar vídeos: {}", e))
                .color(0xFF0000)
                .footer(CreateEmbedFooter::new("Verifique se a YOUTUBE_API_KEY está configurada corretamente"))
                .timestamp(Timestamp::now());

            msg.channel_id
                .send_message(&ctx.http, CreateMessage::new().embed(error_embed))
                .await?;
        }
    }

    Ok(())
}
