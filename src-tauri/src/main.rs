// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest;

#[macro_use]
extern crate dotenvy_macro;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn boot() -> Result<String, String> {
    let client = reqwest::Client::new();
    let api_key = dotenv!("API_KEY");
    let res = client
    .post("https://panel.magmanode.com/api/client/servers/ad87514a-f21a-4c2d-ba5f-11ec86933546/power")
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header("Authorization", format!("Bearer {}", api_key))
    .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
    .json(&serde_json::json!({ "signal": "start" }))
    .send()
    .await; // Add the await keyword here
    if res.is_ok() {
        Ok("Booted!".into())
    } else {
        Err("Failed!".into())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![boot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
