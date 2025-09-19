// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri_plugin_opener::OpenerExt;
use whoami;

#[tauri::command]
fn get_user() -> String {
    let user = whoami::username();
    user
}
#[tauri::command]
pub async fn get_file_path_in_rust_123(file_path: String) {
    println!("The Path Recived :{} ", file_path)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_user,
            get_file_path_in_rust_123
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
