mod cmd;
mod db;

use std::sync::mpsc;
use std::thread::spawn;

use p2p::relay_server::Opt;

fn relay_run(sender: mpsc::Sender<String>) {
    let opt = &Opt { use_ipv6: Some(false), secret_key_seed: 1, port: 4001 };
    spawn(move || {
        let _ = p2p::relay_server::run(opt, sender);
    });
}

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
    relay_run(tx.clone());

    recv(rx);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cmd::greet, cmd::create_key, cmd::generage_key, cmd::insert_msg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
