use super::types::{AIConfig, AIError};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}

const SYSTEM_PROMPT: &str = "You are PEEK assistant. Rules: Answer only what is needed. Maximum 5 lines. Prefer commands/code. No greetings. No unnecessary explanation. Do not say 'Here is'. Do not mention you are AI.";

pub async fn query_openrouter(config: &AIConfig, query: &str) -> Result<String, AIError> {
    if config.api_key.is_empty() {
        return Err(AIError::NotConfigured);
    }

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| AIError::ConnectionFailed(e.to_string()))?;

    let request = ChatRequest {
        model: config.model.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: query.to_string(),
            },
        ],
        max_tokens: config.max_tokens,
    };

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "https://peek.app")
        .header("X-OpenRouter-Title", "PEEK")
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            if e.is_connect() {
                AIError::ConnectionFailed("Check internet connection".to_string())
            } else if e.is_timeout() {
                AIError::Timeout
            } else {
                AIError::ConnectionFailed(e.to_string())
            }
        })?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AIError::ApiError(format!("{}: {}", status, error_text)));
    }

    let chat_response: ChatResponse = response
        .json()
        .await
        .map_err(|e| AIError::ApiError(e.to_string()))?;

    chat_response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or(AIError::NoResponse)
}
