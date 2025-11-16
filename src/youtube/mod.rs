use reqwest::Client;
use serde::Deserialize;
use std::env;

const YOUTUBE_API_BASE: &str = "https://www.googleapis.com/youtube/v3/search";

#[derive(Debug, Deserialize)]
pub struct YouTubeSearchResponse {
    pub items: Vec<YouTubeSearchItem>,
}

#[derive(Debug, Deserialize)]
pub struct YouTubeSearchItem {
    pub id: YouTubeVideoId,
    pub snippet: YouTubeSnippet,
}

#[derive(Debug, Deserialize)]
pub struct YouTubeVideoId {
    #[serde(rename = "videoId")]
    pub video_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YouTubeSnippet {
    pub title: String,
    pub description: String,
    #[serde(rename = "channelTitle")]
    pub channel_title: String,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
}

pub struct YouTubeClient {
    api_key: String,
    http_client: Client,
}

impl YouTubeClient {
    pub fn from_env() -> Result<Self, String> {
        let api_key = env::var("YOUTUBE_API_KEY")
            .map_err(|_| "YOUTUBE_API_KEY não encontrada nas variáveis de ambiente".to_string())?;

        Ok(Self {
            api_key,
            http_client: Client::new(),
        })
    }

    pub async fn search_first_video(&self, query: &str) -> Result<(String, YouTubeSnippet), String> {
        let url = format!(
            "{}?part=snippet&q={}&type=video&maxResults=1&key={}",
            YOUTUBE_API_BASE,
            urlencoding::encode(query),
            self.api_key
        );

        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Erro ao fazer requisição para YouTube API: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("YouTube API retornou erro {}: {}", status, error_text));
        }

        let search_response: YouTubeSearchResponse = response
            .json()
            .await
            .map_err(|e| format!("Erro ao parsear resposta da YouTube API: {}", e))?;

        if search_response.items.is_empty() {
            return Err("Nenhum vídeo encontrado para essa busca".to_string());
        }

        let item = &search_response.items[0];
        let video_id = item.id.video_id.as_ref()
            .ok_or("Vídeo não possui ID válido".to_string())?;

        Ok((video_id.clone(), item.snippet.clone()))
    }

    pub fn get_video_url(video_id: &str) -> String {
        format!("https://www.youtube.com/watch?v={}", video_id)
    }
}
