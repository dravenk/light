// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use crate::db::*;
use util::appdata;
use wallet::phrase::PrivKey;

#[tauri::command]
pub fn greet(name: &str) -> String {
    let msg: &str = "You've been greeted from Rust!";
    println!("{}", msg);
    let conn = sqlite_conn().unwrap();
    let conn = insert_value(conn, name);
    let _ = select_table(&conn.unwrap(), "msg");
    println!("greet! src-tauri/src/main.rs");
    format!("{}{}", msg, name)
}

#[tauri::command]
pub fn get_peer_id() -> String {
    "".to_owned()
}

#[tauri::command]
pub fn create_key(path: &str) -> String {
    println!("Creating a new key to {}", path);
    let _ = wallet::keys::create_priv_k();
    format!("Hello, {}! You've been greeted from Rust!", path)
}

#[tauri::command]
pub fn generage_key(strength: &str) -> String {
    let mnemonic = wallet::phrase::get_mnemonic_by_strength(strength);
    phrase_to_file(&mnemonic);
    mnemonic.phrase
}

#[tauri::command]
pub fn insert_msg(msg: &str) -> String {
    println!("insert_msg, {}! You've been greeted from Rust!", msg);
    let conn = sqlite_conn().unwrap();
    let _ = insert_value(conn, msg);
    format!("Hello, {}! created message!", msg)
}

fn phrase_to_file(priv_key: &PrivKey) {
    let toml_string = toml::to_string(&priv_key).expect("Could not encode TOML value");
    fs::create_dir_all(appdata::app_dir()).expect("create dir error");
    fs::write(appdata::key_path(), toml_string).expect("Could not write to file!");
}
