use tauri::Manager;

#[derive(Clone, serde::Serialize)]
pub struct SendStatusMessagePayload {
    message: String
}

pub fn send_status<M: Into<String> + std::fmt::Display>(window: &tauri::Window, msg: M) {
    println!("STATUS MSG: {}", msg);
    let _ = window.emit_all("send_status_message", SendStatusMessagePayload {
        message: msg.into()
    });

}