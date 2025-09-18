// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use whoami;

#[tauri::command]
fn get_user() -> String {
    let user = whoami::username();
    user
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
