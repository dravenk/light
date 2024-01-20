use p2p::relay_server::{Opt, RelayBehaviour};
use std::{sync::mpsc, thread, thread::spawn};

pub fn relay_run(sender: mpsc::Sender<Box<dyn Fn() + Send>>) {
    let opt = &Opt { use_ipv6: Some(false), secret_key_seed: 1, port: 4001 };
    let (tx, rx): (
        mpsc::Sender<Box<(dyn Fn() -> RelayBehaviour + Send + 'static)>>,
        mpsc::Receiver<Box<dyn Fn() -> RelayBehaviour + Send>>,
    ) = mpsc::channel();
    spawn(move || {
        let _ = p2p::relay_server::run(opt, tx);
    });

    thread::spawn(move || loop {
        println!("等待消息...");
        if let Ok(received) = rx.recv() {
            println!("收到消息...");
            match received() {
                RelayBehaviour::ObservedAddress(address) => {
                    let msg = format!("收到消息... ObservedAddress: {:?}", address);
                    let print_msg = move || {
                        println!("{}", msg);
                    };
                    let _ = sender.send(Box::new(print_msg));
                }
                RelayBehaviour::PeerId(pid) => {
                    let msg = format!("收到消息... PeerId: {:?}", pid);
                    let print_msg = move || {
                        println!("{}", msg);
                    };
                    let _ = sender.send(Box::new(print_msg));
                }
                _ => {}
            }
        }
    });
}

pub fn p2p_init(sender: mpsc::Sender<Box<dyn Fn() + Send>>) {
    spawn(move || p2p::discovered::run(sender.clone()).map_err(|e| eprintln!("Error in main: {}", e)).map(|_| ()));
}

pub fn recv(rx: mpsc::Receiver<Box<dyn Fn() + Send>>) {
    thread::spawn(move || loop {
        println!("等待消息...");
        if let Ok(received) = rx.recv() {
            println!("收到消息...");
            received();
        }
    });
}
