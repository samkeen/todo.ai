// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::todos::{TodoCollection};

mod todos;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn persist_todo(headline: &str, description: &str) -> String {
    // write headline, desc, and isDone to a file as JSON
    let mut todo_collection = TodoCollection::new("../todos.json");
    let new_todo = todo_collection.add(headline, description);

    format!("Task[{}] '{}' recorded", new_todo.id, new_todo.headline)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![persist_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
