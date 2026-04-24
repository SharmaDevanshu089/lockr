const DEBUG: bool = true;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use dirs::desktop_dir;
use rand::RngCore;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tauri::Manager;
use window_vibrancy::apply_mica;

#[derive(Serialize)]
struct FileDialogData {
    filepath: PathBuf,
    filename: String,
}

#[derive(Deserialize, Debug)]
struct SavingDialogData {
    window_title: String,
    file_extension: String,
    file_type_name: String,
}
#[derive(Deserialize, Debug)]
struct EncryptionCommandRequestPackage {
    resultant_dir: PathBuf,
    password: [u8; 32],
    resultname: PathBuf,
    filepath: PathBuf,
    checked: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if DEBUG {
        println!("run is loading");
    }
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "windows")]
            {
                apply_mica(&window, None).expect("Failed to apply acrylic effect");
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            read_nounce_bytes,
            open_in_explorer,
            open_file_dialog,
            open_folder_dialog,
            get_file_path,
            generate_key,
            get_resulting_dir,
            final_encryption,
            final_decryption
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
async fn get_file_path() {
    if DEBUG {
        println!("get_file_path is loading");
    }
    if let Some(path) = FileDialog::new().pick_file() {
        if DEBUG {
            println!("variable path: {:?}", path);
        }
        encript_file_by_path(path);
    }
}
#[tauri::command]
fn open_file_dialog() -> Result<FileDialogData, String> {
    if DEBUG {
        println!("open_file_dialog is loading");
    }
    let fileDialogResult = FileDialog::new()
        .pick_file()
        .ok_or(String::from("FileDialog does not exist"));
    if DEBUG {
        println!("variable fileDialogResult: {:?}", fileDialogResult);
    }
    let filepath = fileDialogResult.clone()?;
    if DEBUG {
        println!("variable filepath: {:?}", filepath);
    }
    let file_name = fileDialogResult
        .clone()?
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    if DEBUG {
        println!("variable file_name: {:?}", file_name);
    }
    let binding = filepath.clone();
    if DEBUG {
        println!("variable binding: {:?}", binding);
    }
    let empty_path_string = binding.to_string_lossy();
    if DEBUG {
        println!("variable empty_path_string: {:?}", empty_path_string);
    }
    if empty_path_string.is_empty() {
        println!("Empty File Found");
        return open_file_dialog();
    }
    let response: FileDialogData = FileDialogData {
        filename: file_name,
        filepath: filepath.clone(),
    };
    if DEBUG {
        println!("return response: {:?}", response.filename); // Only printing name for brevity or could print full struct if derived Debug
        println!("open_file_dialog returns Ok(response)");
    }
    return Ok(response);
}
#[tauri::command]
fn generate_key() -> [u8; 32] {
    if DEBUG {
        println!("generate_key is loading");
    }
    let mut key_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key_bytes);
    if DEBUG {
        println!("variable key_bytes: {:?}", key_bytes);
    }
    println!("{:?}", key_bytes.clone());
    if DEBUG {
        println!("generate_key returns key_bytes");
    }
    return key_bytes;
}

#[tauri::command]
fn get_resulting_dir() -> PathBuf {
    if DEBUG {
        println!("get_resulting_dir is loading");
    }
    let resulting_directory = desktop_dir().unwrap();
    if DEBUG {
        println!("variable resulting_directory: {:?}", resulting_directory);
        println!("get_resulting_dir returns resulting_directory");
    }
    resulting_directory
}

#[tauri::command]
#[allow(deprecated)]
fn final_encryption(responsepackage: EncryptionCommandRequestPackage) -> Result<(), String> {
    if DEBUG {
        println!("final_encryption is loading");
        println!(
            "variable responsepackage is initialsie with {:?}",
            responsepackage
        );
    }
    const BUFFER_SIZE: usize = 4;
    let mut nonce_bytes = [0u8; 12];
    let key_bytes = responsepackage.password;
    if DEBUG {
        println!("variable nonce_bytes: {:?}", nonce_bytes);
        println!("variable key_bytes: {:?}", key_bytes);
    }
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    println!("{:?}", nonce_bytes.clone());
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    if DEBUG {
        println!("variable key: {:?}", key);
        println!("variable nonce: {:?}", nonce);
    }
    let mut path_to_write = responsepackage
        .resultant_dir
        .join(responsepackage.resultname);
    let path_to_read = responsepackage.filepath;
    if DEBUG {
        println!("variable path_to_write: {:?}", path_to_write);
        println!("variable path_to_read: {:?}", path_to_read);
    }
    path_to_write.add_extension("aes");
    if DEBUG {
        println!("variable path_to_write (updated): {:?}", path_to_write);
    }
    let file_to_write =
        fs::File::create(&path_to_write).map_err(|e| format!("File Creation Error{}", e))?;
    let file_to_read =
        fs::File::open(&path_to_read).map_err(|e| format!("File Opening Error{}", e))?;
    if DEBUG {
        println!("variable file_to_write: (File handle)");
        println!("variable file_to_read: (File handle)");
    }
    let mut BufferReader = std::io::BufReader::new(file_to_read);
    let mut BufferWriter = std::io::BufWriter::new(file_to_write);
    if DEBUG {
        println!("variable BufferReader: (handle)");
        println!("variable BufferWriter: (handle)");
    }
    let mut buffer = [0u8; BUFFER_SIZE * 1024];
    let cipher = Aes256Gcm::new(&key);
    if DEBUG {
        println!("variable buffer size: {:?}", buffer.len());
        println!("variable cipher: (created)");
    }
    BufferWriter
        .write_all(&nonce_bytes)
        .map_err(|e| format!("File Write Error{}", e))?;
    loop {
        let n = BufferReader
            .read(&mut buffer)
            .map_err(|e| format!("File Read Error{}", e))?;
        if DEBUG {
            println!("variable n: {:?}", n);
        }
        if n == 0 {
            break;
        }
        let ciphertext = cipher
            .encrypt(nonce, &buffer[..n])
            .map_err(|e| format!("File Encryption Error{}", e))?;
        if DEBUG {
            println!("variable ciphertext length: {:?}", ciphertext.len());
        }
        BufferWriter
            .write_all(&ciphertext)
            .map_err(|e| format!("File Write Error{}", e))?;
    }
    if DEBUG {
        println!("final_encryption returns Ok(())");
    }
    Ok(())
}

#[allow(deprecated)]
fn encript_file_by_path(path_to_file: PathBuf) {
    if DEBUG {
        println!("encript_file_by_path is loading");
        println!(
            "variable path_to_file is initialsie with {:?}",
            path_to_file
        );
    }
    let file = fs::read(&path_to_file).unwrap();
    if DEBUG {
        println!("variable file length: {:?}", file.len());
    }
    let mut key_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key_bytes);
    if DEBUG {
        println!("variable key_bytes: {:?}", key_bytes);
    }

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    if DEBUG {
        println!("variable key: {:?}", key);
    }
    println!("Key generated: {:?}", key);
    let cipher = Aes256Gcm::new(key);
    if DEBUG {
        println!("variable cipher: (created)");
    }
    let mut nonce_bytes = [0u8; 12];
    if DEBUG {
        println!("variable nonce_bytes: {:?}", nonce_bytes);
    }
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    if DEBUG {
        println!("variable nonce_bytes (updated): {:?}", nonce_bytes);
    }
    let nonce = Nonce::from_slice(&nonce_bytes);
    if DEBUG {
        println!("variable nonce: {:?}", nonce);
    }
    println!("Nonce generated: {:?}", nonce);
    let ciphertext = cipher.encrypt(nonce, file.as_ref()).unwrap();
    if DEBUG {
        println!("variable ciphertext length: {:?}", ciphertext.len());
    }
    println!("encrypted: {:?}", ciphertext);
    let mut output = Vec::new();
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);
    let mut path_to_write = path_to_file.clone();
    path_to_write.set_extension("bin");
    let path_to_write_string = path_to_write.to_string_lossy().into_owned();
    if DEBUG {
        println!("variable output length: {:?}", output.len());
        println!("variable path_to_write: {:?}", path_to_write);
        println!("variable path_to_write_string: {:?}", path_to_write_string);
    }
    println!("Path to write: {:?}", path_to_write_string);
    std::fs::write(&path_to_write, output).unwrap();
    //     Now i will remove file
    //     let delete_result = fs::remove_file(&path_to_write).unwrap();
}

#[tauri::command]
fn open_folder_dialog() -> String {
    if DEBUG {
        println!("open_folder_dialog is loading");
    }
    // I really do not have energy to deal with this
    // TODO:Add the proper error handling later on with the project
    let file_url = FileDialog::new().pick_folder().unwrap();
    if DEBUG {
        println!("variable file_url: {:?}", file_url);
    }
    let file_url_string = file_url.to_string_lossy().into_owned();
    if DEBUG {
        println!("variable file_url_string: {:?}", file_url_string);
        println!("open_folder_dialog returns file_url_string");
    }
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

#[tauri::command]
fn open_in_explorer(new_file_path: String) -> Result<(), String> {
    if DEBUG {
        println!("open_in_explorer is loading");
        println!(
            "variable new_file_path is initialsie with {:?}",
            new_file_path
        );
    }
    println!("New File Path: {}", new_file_path);
    let _command_execution = Command::new("explorer.exe")
        .args(&["/select,", &new_file_path])
        .spawn()
        .map_err(|e| format!("Failure: Command , {}", e));
    if DEBUG {
        println!("variable _command_execution: (Result)");
        println!("open_in_explorer returns Ok(())");
    }
    Ok(())
}

#[tauri::command]
fn read_nounce_bytes(path: PathBuf) -> Result<[u8; 12], String> {
    if DEBUG {
        println!("read_nounce_bytes is loading");
        println!("variable path is initialsie with {:?}", path);
    }
    let mut nounce_bytes = [0u8; 12];
    if DEBUG {
        println!("variable nounce_bytes: {:?}", nounce_bytes);
    }
    println!("Reading bytes");
    let mut file_reader = File::open(path).map_err(|_e| "File Open Error")?;
    if DEBUG {
        println!("variable file_reader: (handle)");
    }
    file_reader
        .read_exact(&mut nounce_bytes)
        .map_err(|_e| "File Read Error")?;
    if DEBUG {
        println!("variable nounce_bytes (updated): {:?}", nounce_bytes);
        println!("read_nounce_bytes returns Ok(nounce_bytes)");
    }
    Ok(nounce_bytes)
}

#[tauri::command]
fn open_saving_prompt(output_case_type: String) -> Result<String, String> {
    //Case 1 :For Opening Save as for Saving as Encription target location "type_encryption_save"
    //Case 2: For opening save as for Saving Decryption target location "type_decryption_save
    let window_title: String;
    let file_type: String;
    if output_case_type == "type_encryption_save" {
        println!("open_saving_prompt is loading");
        window_title = "Where to Save Encrypted file".to_string();
        file_type = "AES Encryption File".to_string();
    } else if output_case_type == "type_decryption_save" {
        println!("open_saving_prompt for decryption is loading");
        window_title = "Where to Save Decrypted file".to_string();
        file_type = "AES Encryption File".to_string();
    } else {
        return Err(format!("Unknown output type: {}", output_case_type));
    }
    let file_path_for_saving = FileDialog::new()
        .set_title(&window_title)
        .add_filter(&file_type, &["saving"])
        .save_file();

    match file_path_for_saving {
        Some(path) => Ok(path.to_string_lossy().into_owned()),
        None => Err("No file selected".to_string()),
    }
}

#[tauri::command]
#[allow(deprecated)]
fn final_decryption(responsepackage: EncryptionCommandRequestPackage) -> Result<(), String> {
    if DEBUG {
        println!("final_decryption is loading");
        println!(
            "variable responsepackage is initialsie with {:?}",
            responsepackage
        );
    }
    const BUFFER_SIZE: usize = 4;
    let key_bytes = responsepackage.password;
    if DEBUG {
        println!("variable key_bytes: {:?}", key_bytes);
    }
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    if DEBUG {
        println!("variable key: {:?}", key);
    }
    let path_to_write_base = responsepackage
        .resultant_dir
        .join(responsepackage.resultname);
    // Remove the .aes extension if present, otherwise no change
    let mut path_to_write = path_to_write_base.clone();
    path_to_write.set_extension("");

    let path_to_read = responsepackage.filepath;
    if DEBUG {
        println!("variable path_to_write: {:?}", path_to_write);
        println!("variable path_to_read: {:?}", path_to_read);
    }

    let file_to_read =
        fs::File::open(&path_to_read).map_err(|e| format!("File Opening Error{}", e))?;
    let file_to_write =
        fs::File::create(&path_to_write).map_err(|e| format!("File Creation Error{}", e))?;
    if DEBUG {
        println!("variable file_to_write: (File handle)");
        println!("variable file_to_read: (File handle)");
    }
    let mut BufferReader = std::io::BufReader::new(file_to_read);
    let mut BufferWriter = std::io::BufWriter::new(file_to_write);
    if DEBUG {
        println!("variable BufferReader: (handle)");
        println!("variable BufferWriter: (handle)");
    }

    // Read the nonce first
    let mut nonce_bytes = [0u8; 12];
    BufferReader
        .read_exact(&mut nonce_bytes)
        .map_err(|e| format!("File Read Error (Nonce){}", e))?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    if DEBUG {
        println!("variable nonce_bytes: {:?}", nonce_bytes);
        println!("variable nonce: {:?}", nonce);
    }

    // buffer size is 4096 (original buffer) + 16 (tag size for AES-GCM) = 4112 bytes
    let mut buffer = [0u8; (BUFFER_SIZE * 1024) + 16];
    let cipher = Aes256Gcm::new(&key);
    if DEBUG {
        println!("variable buffer size: {:?}", buffer.len());
        println!("variable cipher: (created)");
    }

    loop {
        // Read in chunks of 4112 bytes precisely to recreate the exact ciphertext blocks produced by encryption
        let mut n = 0;
        while n < buffer.len() {
            match BufferReader.read(&mut buffer[n..]) {
                Ok(0) => break,
                Ok(read_bytes) => n += read_bytes,
                Err(e) => return Err(format!("File Read Error{}", e)),
            }
        }
        if DEBUG {
            println!("variable n: {:?}", n);
        }
        if n == 0 {
            break;
        }
        let plaintext = cipher
            .decrypt(nonce, &buffer[..n])
            .map_err(|e| format!("File Decryption Error{}", e))?;
        if DEBUG {
            println!("variable plaintext length: {:?}", plaintext.len());
        }
        BufferWriter
            .write_all(&plaintext)
            .map_err(|e| format!("File Write Error{}", e))?;
    }
    if DEBUG {
        println!("final_decryption returns Ok(())");
    }
    Ok(())
}
