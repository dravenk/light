mod cmd;
mod db;

use std::sync::mpsc;
use std::thread::spawn;

fn p2p_init(sender: mpsc::Sender<String>) {
    spawn(move || p2p::discovered::run(sender.clone()).map_err(|e| eprintln!("Error in main: {}", e)).map(|_| ()));
}

fn recv(rx: mpsc::Receiver<String>) {
    spawn(move || loop {
        println!("等待消息...");
        if let Result::Ok(received) = rx.recv() {
            println!("收到消息：{}", received);
        }
    });
}

fn main() {
    let _ = db::sqlite_init();

    let (tx, rx) = mpsc::channel();
    p2p_init(tx.clone());
    recv(rx);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cmd::greet, cmd::create_key, cmd::generage_key, cmd::insert_msg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
