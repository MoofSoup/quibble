// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
/* 
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}





fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, save_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}*/

use tauri::api::path::document_dir;
use tauri::command;
#[tauri::command]
fn save_text(content: String, filename: String) -> Result<(), String> {
    let document_path = document_dir().ok_or("Failed to find document directory")?;
    let file_path = document_path.join(filename);
    
    std::fs::write(file_path, content).map_err(|e| e.to_string())
}

#[command]
fn my_simple_function() -> String {
    println!("hello world!");
    "Function called successfully!".to_string()    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_simple_function,save_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}