mod cmd;
mod db;
mod p2p;

use std::sync::mpsc;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = db::sqlite_init();

    // Define the channel to accept Box<dyn Fn() + Send>
        let (tx, rx): (mpsc::Sender<Box<(dyn Fn() + Send + 'static)>>, mpsc::Receiver<Box<dyn Fn() + Send>>) 
            = mpsc::channel();

    p2p::p2p_init(tx.clone());
    p2p::relay_run(tx.clone());
    p2p::recv(rx);

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            cmd::get_status,
            cmd::generage_key,
            cmd::secure_save,
            cmd::secure_load,
            cmd::secure_remove
        ])
        .run(tauri::generate_context!())
        .expect("error while running light client app");
}
