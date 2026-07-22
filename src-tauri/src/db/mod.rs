pub mod schema;

use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(db_path: &PathBuf) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }
}

pub fn get_db_path(app_handle: &AppHandle) -> PathBuf {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("failed to get app data dir");
    fs::create_dir_all(&app_dir).expect("failed to create app data dir");
    app_dir.join("peek.db")
}

pub fn init_database(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = get_db_path(app_handle);
    let db = Database::new(&db_path)?;
    schema::create_tables(&db.conn)?;
    schema::seed_data_if_empty(&db.conn)?;
    Ok(())
}
