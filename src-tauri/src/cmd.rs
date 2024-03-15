// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use account;
use keyring::Entry;
// use account::phrase::PrivKey;
// use light_utils::appdata;

#[tauri::command]
pub fn get_status() -> String {
    "ok".to_owned()
}

#[tauri::command]
pub fn generage_key(_strength: &str) -> String {
    let (phrase, _) = account::seed::get_phrase_seed();
    let phrase_str = phrase.phrase().to_owned().to_string();
    phrase_str
}

#[tauri::command]
pub fn secure_save(key: String, value: String) -> Result<(), ()> {
    let entry = Entry::new("Light", &key).expect("Failed to create entry");
    let _ = entry.set_password(&value);
    Ok(())
}

#[tauri::command]
pub fn secure_load(key: String) -> Result<String, String> {
    let entry = Entry::new("Light", &key).expect("Failed to create entry");
    if let Ok(password) = entry.get_password() {
        Ok(password)
    } else {
        Err("not found".to_string())
    }
}

#[tauri::command]
pub fn secure_remove(key: String) -> Result<(), ()> {
    let entry = Entry::new("Light", &key).expect("Failed to create entry");
    let _ = entry.delete_password();
    Ok(())
}
