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
        .invoke_handler(tauri::generate_handler![open_file_dialog,open_folder_dialog])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
async fn get_file_path() {
    if let Some(path) = FileDialog::new().pick_file() {
        encript_file_by_path(path);
    }
}
#[tauri::command]
fn open_file_dialog() -> String {
    // I really do not have energy to deal with this
    // TODO:Add the proper error handling later on with the project
    let file_url = FileDialog::new().pick_file().unwrap();
    let file_url_string = file_url.to_string_lossy().into_owned();
    // let unfiltered_url = match  {
    //     Some(file_url_string) => file_url_string.to_string(),
    //     None => {
    //         match open_file_dialog(){
    //             Ok(file_url) => file_url_string,
    //             Err(message) => open_file_dialog()
    //         }
    //     }
    // };
    // if unfiltered_url.is_empty() {
    //     Err(String::from("FileNull"))
    // }
    // else {Ok(unfiltered_url)}
    return file_url_string;
}
fn encript_file_by_path(path_to_file: PathBuf) {
    println!("Loading {}", path_to_file.display());
}

#[tauri::command]
fn open_folder_dialog() -> String {
    // I really do not have energy to deal with this
    // TODO:Add the proper error handling later on with the project
    let file_url = FileDialog::new().pick_folder().unwrap();
    let file_url_string = file_url.to_string_lossy().into_owned();
    // let unfiltered_url = match  {
    //     Some(file_url_string) => file_url_string.to_string(),
    //     None => {
    //         match open_file_dialog(){
    //             Ok(file_url) => file_url_string,
    //             Err(message) => open_file_dialog()
    //         }
    //     }
    // };
    // if unfiltered_url.is_empty() {
    //     Err(String::from("FileNull"))
    // }
    // else {Ok(unfiltered_url)}
    return file_url_string;
}