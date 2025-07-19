#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {name}, You have been greeted from Rust!")
}

fn main() {
    tauri_runtime_verso::builder()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            dbg!(app.get_webview_window("main").unwrap().inner_size()).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
