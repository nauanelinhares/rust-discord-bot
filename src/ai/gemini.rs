use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::error::AIError;
use super::provider::AIProvider;

/// Gemini AI Provider implementation
pub struct GeminiProvider {
    api_key: String,
    api_url: String,
    model: String,
    client: Client,
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

impl GeminiProvider {
    /// Create a new Gemini provider with the given API key, URL and model
    pub fn new(api_key: String, api_url: String, model: String) -> Self {
        Self {
            api_key,
            api_url,
            model,
            client: Client::new(),
        }
    }

    /// Create a new Gemini provider from environment variables
    pub fn from_env() -> Result<Self, AIError> {
        let api_key = std::env::var("GEMINI_API_KEY").map_err(|_| {
            AIError::ConfigurationError("GEMINI_API_KEY environment variable not set".to_string())
        })?;

        let api_url = std::env::var("GEMINI_API_URL").unwrap_or_else(|_| {
            "https://generativelanguage.googleapis.com/v1beta/models".to_string()
        });

        let model = std::env::var("GEMINI_MODEL").unwrap_or_else(|_| {
            "gemini-2.5-flash".to_string()
        });

        Ok(Self::new(api_key, api_url, model))
    }
}

#[async_trait]
impl AIProvider for GeminiProvider {
    async fn generate(&self, prompt: &str) -> Result<String, AIError> {
        let url = format!(
            "{}/{}:generateContent",
            self.api_url.trim_end_matches('/'),
            self.model
        );

        let request_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
        };

        let response = self
            .client
            .post(&url)
            .header("x-goog-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AIError::ApiError {
                status,
                message: error_text,
            });
        }

        let gemini_response: GeminiResponse = response.json().await?;

        gemini_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .ok_or(AIError::NoResponse)
    }

    fn name(&self) -> &str {
        "Gemini"
    }
}
