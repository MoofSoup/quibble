// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//mod core;
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
}
//recie

use tauri::api::path::document_dir;
use tauri::command;
#[tauri::command]
fn savetext(content: String, filename: String) -> Result<(), String> {
    let document_path = document_dir().ok_or("Failed to find document directory")?;
    let file_path = document_path.join(filename);
    
    std::fs::write(file_path, content).map_err(|e| e.to_string())
}

#[command]
fn my_simple_function() -> String {
    println!("hello world!");
    "Function called successfully!".to_string()    
}
*/

#[derive(Debug, PartialEq)]
enum AppState {
    DocumentView(String),
    ChatView(String),
}

struct StateMachine {
    current_state: AppState,
}

impl StateMachine {
    fn transition_to_document_view(&mut self, current_state: String) -> String {
        if self.current_state == AppState::ChatView {
            self.current_state = AppState::DocumentView;
            "Document View".to_string()
        } else {
            "Already in Document View".to_string()
        }
    }

    fn transition_to_chat_view(&mut self) -> String {
        if self.current_state == AppState::DocumentView {
            self.current_state = AppState::ChatView;
            "Chat View".to_string()
        } else {
            "Already in Chat View".to_string()
        }
    }

    fn get_inactive_state(&self) -> String {
        match self.current_state {
            AppState::DocumentView => "Chat View".to_string(),
            AppState::ChatView => "Document View".to_string(),
        }
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        StateMachine {
            current_state: AppState::DocumentView,
        }
    }
}

fn main() {
    let mut state_machine = StateMachine::default();

    // Simulate button presses and print the returned state strings
    println!("Current state: {}", state_machine.transition_to_document_view());
    println!("Inactive state: {}", state_machine.get_inactive_state());

    println!("Current state: {}", state_machine.transition_to_chat_view());
    println!("Inactive state: {}", state_machine.get_inactive_state());

    println!("Current state: {}", state_machine.transition_to_chat_view());
    println!("Inactive state: {}", state_machine.get_inactive_state());

    println!("Current state: {}", state_machine.transition_to_document_view());
    println!("Inactive state: {}", state_machine.get_inactive_state());
}


fn main() {
    let chat: &str = r#"<button id='smonyx'>smonyx</button>
    <button id='bigchrome'>bigsmallchrome</button>
    <p>blahdy blah womp womp</p>
    <p>lorem ipsum $ sit amore</p>"#;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

