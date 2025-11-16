use std::fmt;

#[derive(Debug)]
pub enum AIError {
    ApiError { status: u16, message: String },
    NetworkError(String),
    ParseError(String),
    ConfigurationError(String),
    NoResponse,
}

impl fmt::Display for AIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AIError::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            AIError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AIError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AIError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            AIError::NoResponse => write!(f, "No response received from AI provider"),
        }
    }
}

impl std::error::Error for AIError {}

impl From<reqwest::Error> for AIError {
    fn from(err: reqwest::Error) -> Self {
        AIError::NetworkError(err.to_string())
    }
}

impl From<serde_json::Error> for AIError {
    fn from(err: serde_json::Error) -> Self {
        AIError::ParseError(err.to_string())
    }
}
