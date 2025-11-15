use reqwest;
use serde::{Deserialize, Serialize};
use std::env;

const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent";

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

pub async fn ask_gemini(question: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY must be set in environment");

    let url = format!("{}?key={}", GEMINI_API_URL, api_key);

    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: question.to_string(),
            }],
        }],
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!("Gemini API error ({}): {}", status, error_text).into());
    }

    let gemini_response: GeminiResponse = response.json().await?;

    if let Some(candidate) = gemini_response.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            return Ok(part.text.clone());
        }
    }

    Err("No response from Gemini".into())
}
