use sqlx::{
    sqlite::SqlitePoolOptions,
    Pool,
    Sqlite,
};

pub async fn init_db(app: &AppHandle) -> Result<Pool<Sqlite>, sqlx::Error> {

    let dir = app
        .path()
        .app_data_dir()
        .expect("no app data dir");

    std::fs::create_dir_all(&dir).ok();

    let db_path = dir.join("history.db");

    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", db_path.display()))
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