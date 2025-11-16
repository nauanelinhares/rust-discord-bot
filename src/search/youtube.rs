use async_trait::async_trait;
use regex::Regex;
use serde_json::Value;

use crate::search::error::SearchError;
use crate::search::Search;

#[derive(Debug, Clone)]
pub struct YoutubeVideo {
    pub id: Option<String>,
    pub thumbnails: Vec<String>,
    pub title: Option<String>,
    pub long_desc: Option<String>,
    pub channel: Option<String>,
    pub duration: Option<String>,
    pub views: Option<String>,
    pub publish_time: Option<String>,
    pub url_suffix: Option<String>,
}

pub struct YoutubeSearch {}

impl YoutubeSearch {
    pub fn new() -> Self {
        Self {}
    }

    fn extract_json_from_html(html: &str) -> Result<Value, SearchError> {
        let re = Regex::new(r"var ytInitialData = (\{.*?\});")
            .map_err(|e| SearchError::ParseError(format!("Failed to create regex: {}", e)))?;

        let mut json_str = String::new();
        let mut brace_count = 0;
        let mut in_json = false;
        let mut start_pos = 0;

        if let Some(captures) = re.captures(html) {
            if let Some(matched) = captures.get(0) {
                let matched_str = matched.as_str();
                if let Some(brace_start) = matched_str.find('{') {
                    start_pos = matched.start() + brace_start;
                    in_json = true;
                }
            }
        }

        if !in_json {
            return Err(SearchError::ParseError(
                "Could not find ytInitialData in HTML".to_string(),
            ));
        }

        for (_i, ch) in html[start_pos..].char_indices() {
            match ch {
                '{' => {
                    brace_count += 1;
                    json_str.push(ch);
                }
                '}' => {
                    brace_count -= 1;
                    json_str.push(ch);
                    if brace_count == 0 {
                        break;
                    }
                }
                _ => {
                    if in_json {
                        json_str.push(ch);
                    }
                }
            }
        }

        if brace_count != 0 {
            return Err(SearchError::ParseError(
                "Invalid JSON structure in HTML".to_string(),
            ));
        }

        let json: Value = serde_json::from_str(&json_str)?;
        Ok(json)
    }

    fn extract_videos_from_json(json: &Value) -> Vec<YoutubeVideo> {
        let mut results = Vec::new();

        if let Some(contents) = json
            .get("contents")
            .and_then(|c| c.get("twoColumnSearchResultsRenderer"))
            .and_then(|t| t.get("primaryContents"))
            .and_then(|p| p.get("sectionListRenderer"))
            .and_then(|s| s.get("contents"))
            .and_then(|c| c.as_array())
        {
            for content in contents {
                if let Some(item_section) = content.get("itemSectionRenderer") {
                    if let Some(items) = item_section.get("contents").and_then(|c| c.as_array()) {
                        for item in items {
                            if let Some(video_renderer) = item.get("videoRenderer") {
                                let video = YoutubeVideo {
                                    id: video_renderer
                                        .get("videoId")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string()),
                                    thumbnails: video_renderer
                                        .get("thumbnail")
                                        .and_then(|t| t.get("thumbnails"))
                                        .and_then(|t| t.as_array())
                                        .map(|arr| {
                                            arr.iter()
                                                .filter_map(|thumb| {
                                                    thumb.get("url").and_then(|u| u.as_str())
                                                })
                                                .map(|s| s.to_string())
                                                .collect()
                                        })
                                        .unwrap_or_default(),
                                    title: video_renderer
                                        .get("title")
                                        .and_then(|t| t.get("runs"))
                                        .and_then(|r| r.as_array())
                                        .and_then(|arr| arr.get(0))
                                        .and_then(|r| r.get("text"))
                                        .and_then(|t| t.as_str())
                                        .map(|s| s.to_string()),
                                    long_desc: video_renderer
                                        .get("descriptionSnippet")
                                        .and_then(|d| d.get("runs"))
                                        .and_then(|r| r.as_array())
                                        .and_then(|arr| arr.get(0))
                                        .and_then(|r| r.get("text"))
                                        .and_then(|t| t.as_str())
                                        .map(|s| s.to_string()),
                                    channel: video_renderer
                                        .get("longBylineText")
                                        .and_then(|l| l.get("runs"))
                                        .and_then(|r| r.as_array())
                                        .and_then(|arr| arr.get(0))
                                        .and_then(|r| r.get("text"))
                                        .and_then(|t| t.as_str())
                                        .map(|s| s.to_string()),
                                    duration: video_renderer
                                        .get("lengthText")
                                        .and_then(|l| l.get("simpleText"))
                                        .and_then(|t| t.as_str())
                                        .map(|s| s.to_string()),
                                    views: video_renderer
                                        .get("viewCountText")
                                        .and_then(|v| v.get("simpleText"))
                                        .and_then(|t| t.as_str())
                                        .map(|s| s.to_string()),
                                    publish_time: video_renderer
                                        .get("publishedTimeText")
                                        .and_then(|p| p.get("simpleText"))
                                        .and_then(|t| t.as_str())
                                        .map(|s| s.to_string()),
                                    url_suffix: video_renderer
                                        .get("navigationEndpoint")
                                        .and_then(|n| n.get("commandMetadata"))
                                        .and_then(|c| c.get("webCommandMetadata"))
                                        .and_then(|w| w.get("url"))
                                        .and_then(|u| u.as_str())
                                        .map(|s| s.to_string()),
                                };
                                results.push(video);
                            }
                        }
                    }
                }
            }
        }

        results
    }
}

#[async_trait]
impl Search for YoutubeSearch {
    async fn search(&self, query: &str) -> Result<Vec<YoutubeVideo>, SearchError> {
        let url = format!(
            "https://www.youtube.com/results?search_query={}",
            urlencoding::encode(query)
        );
        let response = reqwest::get(&url).await?;
        let body = response.text().await?;

        let json = Self::extract_json_from_html(&body)?;
        let videos = Self::extract_videos_from_json(&json);

        Ok(videos)
    }
}
