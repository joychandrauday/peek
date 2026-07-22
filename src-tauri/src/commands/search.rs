use crate::ai::query_openrouter;
use crate::ai::types::AIConfig;
use crate::db::{get_db_path, Database};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: i64,
    pub title: String,
    pub answer: String,
    pub category: String,
    pub tags: Option<String>,
    pub score: f64,
}

fn search_local(db: &Database, query: &str) -> Result<Vec<SearchResult>, String> {
    let mut stmt = db
        .conn
        .prepare("SELECT id, title, answer, category, tags FROM commands")
        .map_err(|e| e.to_string())?;

    let commands: Vec<(i64, String, String, String, Option<String>)> = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    drop(stmt);

    let query_lower = query.to_lowercase();
    let query_words: Vec<&str> = query_lower.split_whitespace().collect();

    let mut results: Vec<SearchResult> = commands
        .iter()
        .filter_map(|(id, title, answer, category, tags)| {
            let title_lower = title.to_lowercase();
            let answer_lower = answer.to_lowercase();
            let tags_lower = tags.as_deref().unwrap_or("").to_lowercase();

            let mut score = 0.0;

            if title_lower == query_lower {
                score = 100.0;
            } else if title_lower.starts_with(&query_lower) {
                score = 90.0;
            } else if title_lower.contains(&query_lower) {
                score = 80.0;
            } else if answer_lower.contains(&query_lower) {
                score = 70.0;
            } else if tags_lower.contains(&query_lower) {
                score = 60.0;
            } else {
                let matched_words = query_words
                    .iter()
                    .filter(|word| {
                        title_lower.contains(*word)
                            || answer_lower.contains(*word)
                            || tags_lower.contains(*word)
                    })
                    .count();

                if matched_words > 0 {
                    score = (matched_words as f64 / query_words.len() as f64) * 50.0;
                }
            }

            if score > 0.0 {
                Some(SearchResult {
                    id: *id,
                    title: title.clone(),
                    answer: answer.clone(),
                    category: category.clone(),
                    tags: tags.clone(),
                    score,
                })
            } else {
                None
            }
        })
        .collect();

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    results.truncate(10);

    if let Some(top) = results.first() {
        let _ = db.conn.execute(
            "UPDATE commands SET usage_count = usage_count + 1 WHERE id = ?1",
            params![top.id],
        );
    }

    Ok(results)
}

#[tauri::command]
pub async fn search_commands(
    query: String,
    ai_config: Option<AIConfig>,
    app_handle: AppHandle,
) -> Result<Vec<SearchResult>, String> {
    let db_path = get_db_path(&app_handle);
    let db = Database::new(&db_path).map_err(|e| e.to_string())?;

    let mut results = search_local(&db, &query)?;

    drop(db);

    let should_use_ai = results.is_empty()
        || results.first().map(|r| r.score < 50.0).unwrap_or(false);

    if should_use_ai {
        if let Some(config) = ai_config {
            if !config.api_key.is_empty() {
                if let Ok(ai_answer) = query_openrouter(&config, &query).await {
                    results.insert(
                        0,
                        SearchResult {
                            id: -1,
                            title: "AI Answer".to_string(),
                            answer: ai_answer,
                            category: "ai".to_string(),
                            tags: Some("ai,generated".to_string()),
                            score: 95.0,
                        },
                    );
                }
            }
        }
    }

    Ok(results)
}
