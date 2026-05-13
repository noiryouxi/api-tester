// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod engine;
mod commands;

use engine::db::init_db;
use commands::http::send_request;
use tauri::Manager;
use sqlx::{
    Pool,
    Sqlite,
};


#[tokio::main]
async fn main() {
    

    tauri::Builder::default()

        // 전역 state 등록
        .manage(db)

        .setup(|app| {
            // SQLite 초기화
            let db = tauri::async_runtime::block_on(
                engine::db::init_db(app.handle())
            ).expect("Failed to init database");

            let window = app.get_webview_window("main").unwrap();

            window.set_size(tauri::Size::Logical(
                tauri::LogicalSize {
                    width: 1200.0,
                    height: 800.0,
                },
            ))?;

            Ok(())
        })

        .invoke_handler(
            tauri::generate_handler![
                send_request
            ]
        )

        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}