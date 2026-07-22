use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub api_key: String,
    pub model: String,
    pub max_tokens: u32,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "openrouter/free".to_string(),
            max_tokens: 200,
        }
    }
}

#[derive(Debug)]
pub enum AIError {
    NotConfigured,
    ConnectionFailed(String),
    ApiError(String),
    Timeout,
    NoResponse,
}

impl std::fmt::Display for AIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIError::NotConfigured => write!(f, "AI not configured. Add API key in settings."),
            AIError::ConnectionFailed(e) => write!(f, "Connection failed: {}", e),
            AIError::ApiError(e) => write!(f, "API error: {}", e),
            AIError::Timeout => write!(f, "Request timed out"),
            AIError::NoResponse => write!(f, "No response from AI"),
        }
    }
}
