use serenity::all::standard::Args;
use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::search::{Search, YoutubeSearch};

#[command]
async fn play_music(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.rest();

    if query.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Por favor, forneça um termo de busca.")
            .await?;
        return Ok(());
    }

    msg.channel_id.broadcast_typing(&ctx.http).await?;

    let search = YoutubeSearch::new();
    let videos = match search.search(query).await {
        Ok(videos) => videos,
        Err(e) => {
            msg.channel_id
                .say(&ctx.http, format!("Erro ao buscar vídeos: {}", e))
                .await?;
            return Ok(());
        }
    };

    if videos.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Nenhum vídeo encontrado para essa busca.")
            .await?;
        return Ok(());
    }

    let mut description = String::new();
    for (index, video) in videos.iter().take(10).enumerate() {
        let title = video.title.as_deref().unwrap_or("Sem título");
        let channel = video.channel.as_deref().unwrap_or("Canal desconhecido");
        let duration = video.duration.as_deref().unwrap_or("N/A");
        let views = video.views.as_deref().unwrap_or("N/A");

        let video_url = if let Some(id) = &video.id {
            format!("https://www.youtube.com/watch?v={}", id)
        } else {
            "Link não disponível".to_string()
        };

        description.push_str(&format!("**{}. {}**\n", index + 1, title));
        description.push_str(&format!("👤 {}\n", channel));
        description.push_str(&format!("⏱️ {} | 👁️ {}\n", duration, views));
        description.push_str(&format!("🔗 {}\n\n", video_url));
    }

    if videos.len() > 10 {
        description.push_str(&format!(
            "\n*Mostrando 10 de {} resultados encontrados*",
            videos.len()
        ));
    }

    let embed = CreateEmbed::new()
        .title("🎵 Resultados da Busca no YouTube")
        .description(&description)
        .color(0xFF0000)
        .field("🔍 Busca", query, false)
        .field("👤 Solicitado por", msg.author.name.clone(), true)
        .footer(CreateEmbedFooter::new(
            "Use !play_music <termo> para buscar músicas",
        ))
        .timestamp(Timestamp::now());

    msg.channel_id
        .send_message(&ctx.http, CreateMessage::new().embed(embed))
        .await?;

    Ok(())
}
