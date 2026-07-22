use crate::db::{get_db_path, Database};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub id: i64,
    pub title: String,
    pub answer: String,
    pub category: String,
    pub tags: Option<String>,
    pub source_url: Option<String>,
    pub usage_count: i64,
    pub created_at: String,
}

#[tauri::command]
pub fn get_all_commands(app_handle: AppHandle) -> Result<Vec<Command>, String> {
    let db_path = get_db_path(&app_handle);
    let db = Database::new(&db_path).map_err(|e| e.to_string())?;

    let mut stmt = db
        .conn
        .prepare(
            "SELECT id, title, answer, category, tags, source_url, usage_count, created_at 
             FROM commands ORDER BY usage_count DESC",
        )
        .map_err(|e| e.to_string())?;

    let commands = stmt
        .query_map([], |row| {
            Ok(Command {
                id: row.get(0)?,
                title: row.get(1)?,
                answer: row.get(2)?,
                category: row.get(3)?,
                tags: row.get(4)?,
                source_url: row.get(5)?,
                usage_count: row.get(6)?,
                created_at: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(commands)
}

#[tauri::command]
pub fn add_command(
    title: String,
    answer: String,
    category: String,
    tags: Option<String>,
    app_handle: AppHandle,
) -> Result<i64, String> {
    let db_path = get_db_path(&app_handle);
    let db = Database::new(&db_path).map_err(|e| e.to_string())?;

    db.conn
        .execute(
            "INSERT INTO commands (title, answer, category, tags) VALUES (?1, ?2, ?3, ?4)",
            params![title, answer, category, tags],
        )
        .map_err(|e| e.to_string())?;

    Ok(db.conn.last_insert_rowid())
}

#[tauri::command]
pub fn delete_command(id: i64, app_handle: AppHandle) -> Result<(), String> {
    let db_path = get_db_path(&app_handle);
    let db = Database::new(&db_path).map_err(|e| e.to_string())?;

    db.conn
        .execute("DELETE FROM commands WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    Ok(())
}
