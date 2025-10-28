// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rfd::FileDialog;
//use std::path::Path;
use std::path::PathBuf;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_file_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
async fn get_file_path() {
    if let Some(path) = FileDialog::new().pick_file() {
        encript_file_by_path(path);
    }
}
fn encript_file_by_path(path_to_file: PathBuf) {}
