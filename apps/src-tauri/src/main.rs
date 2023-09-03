// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use axum::{
    body,
    http::{self, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let router = Router::new().route("/", get(|| async { "Hello, World!" }));
    let port = "3001";
    let host = format!("0.0.0.0:{:}", port);
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::spawn(async move {
                axum::Server::bind(&host.to_string().parse().unwrap())
                    .serve(router.into_make_service())
                    .await
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
