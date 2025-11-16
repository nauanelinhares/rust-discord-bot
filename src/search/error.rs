use std::fmt;

#[derive(Debug)]
pub enum SearchError {
    NetworkError(String),
    ParseError(String),
    NotFound(String),
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            SearchError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            SearchError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for SearchError {}

impl From<reqwest::Error> for SearchError {
    fn from(err: reqwest::Error) -> Self {
        SearchError::NetworkError(err.to_string())
    }
}

impl From<serde_json::Error> for SearchError {
    fn from(err: serde_json::Error) -> Self {
        SearchError::ParseError(err.to_string())
    }
}
