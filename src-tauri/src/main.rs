// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod graph_json;
mod graph;
mod messages;

use graph_json::GraphJson;
use messages::send_status;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn compile_graph(window: tauri::Window, graph_payload: String) {
    send_status(&window, "Beginning graph compilation");

    let graph: serde_json::Result<GraphJson> = serde_json::from_str(&graph_payload);
    if graph.is_err() {
        send_status(&window, format!("Error deserializing graph: {:?}", graph.unwrap_err()));
        return;
    }
    
    let graph = graph.unwrap();

    send_status(&window, "Compiled graph!");
}

#[tauri::command]
fn play(window: tauri::Window) {
    send_status(&window, "Playing music!");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rita!", name)
}

#[tauri::command]
async fn pickFile() -> Option<String> {
    use tauri::api::dialog::blocking::FileDialogBuilder;

    let dialog_result = FileDialogBuilder::new().pick_file();
    dialog_result.map(|pb| pb.to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![pickFile, compile_graph, play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
