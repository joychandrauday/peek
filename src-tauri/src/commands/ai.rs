use crate::ai::query_openrouter;
use crate::ai::types::AIConfig;

#[tauri::command]
pub async fn query_ai(query: String, config: AIConfig) -> Result<String, String> {
    query_openrouter(&config, &query)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_ai_connection(config: AIConfig) -> Result<String, String> {
    let result = query_ai("What is 2+2?".to_string(), config).await?;
    Ok(format!("Connected! Response: {}", result))
}
