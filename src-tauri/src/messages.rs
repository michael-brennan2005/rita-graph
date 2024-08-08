use tauri::Manager;

#[derive(Clone, serde::Serialize)]
pub struct SendStatusMessage {
    message: String
}

pub fn send_status<M: Into<String> + std::fmt::Display>(window: &tauri::Window, msg: M) {
    println!("STATUS MSG: {}", msg);
    let _ = window.emit_all("send_status_message", SendStatusMessage {
        message: msg.into()
    });

}

#[derive(Clone, serde::Serialize)]
pub struct UpdatePlaybackPosition {
    pub current_seconds: usize,
    pub total_seconds: usize
}
