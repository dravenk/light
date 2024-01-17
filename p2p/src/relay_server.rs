use clap::Parser;
use futures::executor::block_on;
use futures::stream::StreamExt;
use libp2p::{
    core::multiaddr::Protocol,
    core::Multiaddr,
    identify, identity, noise, ping, relay,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};

use std::net::{Ipv4Addr, Ipv6Addr};
use std::{error::Error, sync::mpsc::Sender};
use tracing_subscriber::EnvFilter;

pub fn main() {
    // let opt = Opt::parse();
    let opt = &Opt { use_ipv6: Some(false), secret_key_seed: 1, port: 4001 };

    let (tx, _) = std::sync::mpsc::channel();
    let _ = self::run(opt, tx);
}

pub fn run(opt: &Opt, sender: Sender<Box<dyn Fn() + Send>>) -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).try_init();
    // Create a static known PeerId based on given secret
    let local_key: identity::Keypair = generate_ed25519(opt.secret_key_seed);

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_async_std()
        .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default)?
        .with_quic()
        .with_behaviour(|key| Behaviour {
            relay: relay::Behaviour::new(key.public().to_peer_id(), Default::default()),
            ping: ping::Behaviour::new(ping::Config::new()),
            identify: identify::Behaviour::new(identify::Config::new("/TODO/0.0.1".to_string(), key.public())),
        })?
        .build();

    // Listen on all interfaces
    let listen_addr_tcp = Multiaddr::empty()
        .with(match opt.use_ipv6 {
            Some(true) => Protocol::from(Ipv6Addr::UNSPECIFIED),
            _ => Protocol::from(Ipv4Addr::UNSPECIFIED),
        })
        .with(Protocol::Tcp(opt.port));
    swarm.listen_on(listen_addr_tcp)?;

    let listen_addr_quic = Multiaddr::empty()
        .with(match opt.use_ipv6 {
            Some(true) => Protocol::from(Ipv6Addr::UNSPECIFIED),
            _ => Protocol::from(Ipv4Addr::UNSPECIFIED),
        })
        .with(Protocol::Udp(opt.port))
        .with(Protocol::QuicV1);
    swarm.listen_on(listen_addr_quic)?;

    block_on(async {
        loop {
            match swarm.next().await.expect("Infinite Stream.") {
                SwarmEvent::Behaviour(event) => {
                    if let BehaviourEvent::Identify(identify::Event::Received { info: identify::Info { observed_addr, .. }, .. }) =
                        &event
                    {
                        swarm.add_external_address(observed_addr.clone());
                        let msg: String = format!("relay_server: Observed address: {:?}", observed_addr.clone());
                        let print_msg = move || {
                            println!("{}", msg);
                        };
                        let _ = sender.send(Box::new(print_msg));
                    }

                    println!("{event:?}")
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    // println!("relay_server: Listening on {address:?}");
                    // println!("relay_server: Listening on {:?}", swarm.local_peer_id());
                    let msg: String = format!("relay_server: Listening on: {:?}", address.clone());
                    let print_msg = move || {
                        println!("{}", msg);
                    };
                    let _ = sender.send(Box::new(print_msg));

                    let msg: String = format!("relay_server: Listening on local peer {:?}", swarm.local_peer_id().to_string());
                    let print_msg = move || {
                        println!("{}", msg);
                    };
                    let _ = sender.send(Box::new(print_msg));
                }
                _ => {}
            }
        }
    })
}

#[derive(NetworkBehaviour)]
struct Behaviour {
    relay: relay::Behaviour,
    ping: ping::Behaviour,
    identify: identify::Behaviour,
}

fn generate_ed25519(secret_key_seed: u8) -> identity::Keypair {
    let mut bytes = [0u8; 32];
    bytes[0] = secret_key_seed;

    identity::Keypair::ed25519_from_bytes(bytes).expect("only errors on wrong length")
}

#[derive(Debug, Parser)]
#[clap(name = "libp2p relay")]
pub struct Opt {
    /// Determine if the relay listen on ipv6 or ipv4 loopback address. the default is ipv4
    #[clap(long)]
    pub use_ipv6: Option<bool>,

    /// Fixed value to generate deterministic peer id
    #[clap(long)]
    pub secret_key_seed: u8,

    /// The port used to listen on all interfaces
    #[clap(long)]
    pub port: u16,
}
