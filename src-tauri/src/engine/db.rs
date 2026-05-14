use sqlx::{
    Pool,
    Sqlite,
    sqlite::{
        SqlitePoolOptions,
        SqliteConnectOptions,
    },
};

use tauri::{
    AppHandle,
    Manager,
};

pub async fn init_db(
    _app: &AppHandle
) -> Result<Pool<Sqlite>, sqlx::Error> {

    let db_path = std::env::var("APPDATA")
        .map(|p| std::path::PathBuf::from(p))
        .unwrap()
        .join("api-tester")
        .join("history.db");

    println!("DB PATH: {:?}", db_path);

    std::fs::create_dir_all(db_path.parent().unwrap())
        .expect("Failed to create db directory");

    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS http_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            method TEXT NOT NULL,
            url TEXT NOT NULL,
            request_headers TEXT,
            request_body TEXT,
            status INTEGER,
            response_headers TEXT,
            response_body TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}