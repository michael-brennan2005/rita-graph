// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod graph_json;
mod playback;
mod graph;
mod messages;

use std::sync::{Arc, Mutex};

use graph::AudioGraph;
use graph_json::GraphJson;
use messages::send_status;
use playback::{player::Player, spec::F32FormatSpec};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn compile_graph(window: tauri::Window, app_state: tauri::State<'_, AppState>, graph_payload: String) -> Result<(), ()> {
    send_status(&window, "Beginning graph compilation");

    let graph: serde_json::Result<GraphJson> = serde_json::from_str(&graph_payload);
    if graph.is_err() {
        send_status(&window, format!("Error deserializing graph: {:?}", graph.unwrap_err()));
        return Err(());
    }
    
    let mut graph: AudioGraph = AudioGraph::try_from(graph.unwrap()).unwrap(); 
    let format: F32FormatSpec = {
        let player = app_state.player.lock().expect("Failed to lock mutex");
        player.format()
    };

    match graph.process(&window, format) {
        Ok(buf) => {
            send_status(&window, "Uploading to player...");   

            let mut player = app_state.player.lock().expect("Mutex lock failed");
            player.set_new_buffer(buf);

            send_status(&window, "Graph compilation success!");   
        },
        Err(_) => {
            send_status(&window, "Graph compilation failed");
        },
    }

    Ok(())
}

#[tauri::command]
fn play(app_state: tauri::State<'_, AppState>) {
    let mut player = app_state.player.lock().expect("Mutex lock failed");
    player.play();
}

#[tauri::command]
fn pause(app_state: tauri::State<'_, AppState>) {
    let mut player = app_state.player.lock().expect("Mutex lock failed");
    player.pause();
}

#[tauri::command]
async fn pick_file() -> Option<String> {
    use tauri::api::dialog::blocking::FileDialogBuilder;

    let dialog_result = FileDialogBuilder::new().pick_file();
    dialog_result.map(|pb| pb.to_string_lossy().to_string())
}

struct AppState {
    pub player: Arc<Mutex<Player>>
}

// Rust is the worst lang, except for all the other ones 
unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

impl AppState {
    pub fn new() -> Self {
        Self {
            player: Arc::new(Mutex::new(Player::new())),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![pick_file, compile_graph, play, pause])
        // .setup(|app| {
        //     let app_handle = app.handle();

        //     this is super buggy
        //     std::thread::spawn(move || {
        //         let app_state = app_handle.state::<AppState>();

        //         loop {
        //             let mut player = app_state.player.lock().expect("Cannot lock mutex");
                    
        //             while let Ok(msg) = player.pop_message() {
        //                 match msg {
        //                     PlayerToAppMessage::PlaybackPosition(current_frame, total_frames) => {
        //                         let current_seconds = current_frame / player.format().sample_rate;
        //                         let total_seconds = total_frames / player.format().sample_rate;
                                
        //                         let _ = app_handle.emit_all("update_playback_position", UpdatePlaybackPosition {
        //                             current_seconds,
        //                             total_seconds,
        //                         });
        //                     },
        //                     _ => {},
        //                 }
        //             }
    
        //             std::thread::sleep(Duration::from_secs_f32(1.0 / 60.0));    
        //         }
        //     });

        //     Ok(())
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
