use serenity::all::standard::Args;
use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::AIProviderContainer;

// Discord embed description limit
const EMBED_DESCRIPTION_LIMIT: usize = 4096;
const TRUNCATE_SUFFIX: &str = "\n\n... *(resposta truncada por exceder o limite)*";

#[command]
async fn answer(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let question = args.rest();

    if question.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Por favor, faça uma pergunta!")
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

    // Add synthesis instruction to help Gemini provide concise responses
    let enhanced_question = format!(
        "Responda de forma clara e objetiva, limitando sua resposta a aproximadamente 3000 caracteres. \
        Se a resposta for muito longa, priorize as informações mais importantes e relevantes.\n\n\
        Pergunta: {}",
        question
    );

    let answer = match provider.generate(&enhanced_question).await {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("Error calling AI provider ({}): {}", provider.name(), e);
            format!(
                "Desculpe, não consegui obter uma resposta do {}: {}",
                provider.name(),
                e
            )
        }
    };

    // Truncate answer if it exceeds Discord's embed description limit
    let truncated_answer = if answer.len() > EMBED_DESCRIPTION_LIMIT {
        let max_len = EMBED_DESCRIPTION_LIMIT - TRUNCATE_SUFFIX.len();
        format!("{}{}", &answer[..max_len], TRUNCATE_SUFFIX)
    } else {
        answer
    };

    // Create a beautiful embed response
    let embed = CreateEmbed::new()
        .title("🤖 Resposta do AI Assistant")
        .description(&truncated_answer)
        .color(0x5865F2) // Discord blurple color
        .field("❓ Pergunta", question, false)
        .field("👤 Solicitado por", msg.author.name.clone(), true)
        .field("🔧 Powered by", provider.name(), true)
        .footer(CreateEmbedFooter::new("Use !answer <pergunta> para fazer mais perguntas"))
        .timestamp(Timestamp::now());

    // Send the message with embed
    msg.channel_id
        .send_message(&ctx.http, CreateMessage::new().embed(embed))
        .await?;

    Ok(())
}
