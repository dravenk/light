mod cmd;
mod db;
mod p2p;

use std::sync::mpsc;

fn main() {
    let _ = db::sqlite_init();

    // Define the channel to accept Box<dyn Fn() + Send>
    let (tx, rx): (mpsc::Sender<Box<(dyn Fn() + Send + 'static)>>, mpsc::Receiver<Box<dyn Fn() + Send>>) = mpsc::channel();
    p2p::p2p_init(tx.clone());
    p2p::relay_run(tx.clone());
    p2p::recv(rx);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cmd::greet, cmd::get_peer_id, cmd::create_key, cmd::generage_key, cmd::insert_msg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
