mod cmd;
mod db;

use p2p::relay_server::Opt;
use std::{sync::mpsc, thread, thread::spawn};

fn relay_run(sender: mpsc::Sender<Box<dyn Fn() + Send>>) {
    let opt = &Opt { use_ipv6: Some(false), secret_key_seed: 1, port: 4001 };
    spawn(move || {
        let _ = p2p::relay_server::run(opt, sender);
    });
}

fn p2p_init(sender: mpsc::Sender<Box<dyn Fn() + Send>>) {
    spawn(move || p2p::discovered::run(sender.clone()).map_err(|e| eprintln!("Error in main: {}", e)).map(|_| ()));
}

fn recv(rx: mpsc::Receiver<Box<dyn Fn() + Send>>) {
    thread::spawn(move || loop {
        println!("等待消息...");
        if let Ok(received) = rx.recv() {
            println!("收到消息...");
            received();
        }
    });
}

fn main() {
    let _ = db::sqlite_init();

    // Define the channel to accept Box<dyn Fn() + Send>
    let (tx, rx): (mpsc::Sender<Box<dyn Fn() + Send>>, mpsc::Receiver<Box<dyn Fn() + Send>>) = mpsc::channel();
    p2p_init(tx.clone());
    relay_run(tx.clone());
    recv(rx);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cmd::greet, cmd::create_key, cmd::generage_key, cmd::insert_msg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
