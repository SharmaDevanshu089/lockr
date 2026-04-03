use std::fs;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::RngCore;
use rfd::FileDialog;
use window_vibrancy::apply_mica;
use tauri::Manager;
use std::fs::{read, File};
mod encrypt;
//use std::path::Path;
use std::path::PathBuf;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "windows")]
            {
                // The second parameter is an optional RGBA tint color: (R, G, B, Alpha)
                // Adjust these values to match your app's theme
                apply_mica(&window,None).expect("Failed to apply acrylic effect");
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_file_dialog,open_folder_dialog,get_file_path])
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
    let file = fs::read(&path_to_file).unwrap();
    let mut key_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key_bytes);

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    println!("Key generated: {:?}", key);
    let cipher = Aes256Gcm::new(key);
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    println!("Nonce generated: {:?}", nonce);
    let ciphertext = cipher.encrypt(nonce, file.as_ref()).unwrap();
    println!("encrypted: {:?}", ciphertext);
    let mut output = Vec::new();
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);
    let mut path_to_write = &mut path_to_file.clone();
    path_to_write.set_extension("bin");
    let path_to_write_string = path_to_write.to_string_lossy().into_owned();
    println!("Path to write: {:?}", path_to_write_string);
    std::fs::write(&path_to_write, output).unwrap();
//     Now i will remove file
//     let delete_result = fs::remove_file(&path_to_write).unwrap();
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