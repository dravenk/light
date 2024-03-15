use futures::future::Either;
use futures::StreamExt;
use libp2p::{
    core::{muxing::StreamMuxerBox, transport::OrTransport, upgrade::Version},
    identity, mdns, noise, quic, request_response,
    request_response::{Message, ProtocolSupport},
    swarm::{Config, NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId, StreamProtocol, Swarm, Transport,
};
use serde::{Deserialize, Serialize};
use tracing::debug;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct FileRequest(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FileResponse(Vec<u8>);

#[derive(NetworkBehaviour)]
struct NodeBehaviour {
    mdns: mdns::async_io::Behaviour,                                       // 结点发现
    handler: request_response::cbor::Behaviour<FileRequest, FileResponse>, // 数据传输
}

use std::sync::mpsc;
use std::thread;

fn new_swarm() -> Swarm<NodeBehaviour> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    debug!("Local peer id: {:?}", local_peer_id.to_string());

    let tcp_transport = tcp::async_io::Transport::new(tcp::Config::default().nodelay(true))
        .upgrade(Version::V1Lazy)
        .authenticate(noise::Config::new(&local_key).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    let quic_transport = quic::async_std::Transport::new(quic::Config::new(&local_key));
    let transport = OrTransport::new(quic_transport, tcp_transport)
        .map(|either_output, _| match either_output {
            Either::Left((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
            Either::Right((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
        })
        .boxed();

    let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), local_peer_id).unwrap();
    let stream_protocal = "/file-exchange/2";
    let request_handler = request_response::cbor::Behaviour::new(
        [(StreamProtocol::new(stream_protocal), ProtocolSupport::Full)],
        request_response::Config::default(),
    );
    //   keep_live: Config::new(Duration::from_secs(u64::MAX))
    let behaviour = NodeBehaviour {
        mdns,
        handler: request_handler,
        // keey_live: Behaviour,
    };

    let swarm: Swarm<NodeBehaviour> = Swarm::new(
        transport,
        behaviour,
        local_peer_id,
        Config::with_tokio_executor().with_idle_connection_timeout(std::time::Duration::from_secs(u64::MAX)),
    );
    swarm
}

async fn swarm_process(mut swarm: Swarm<NodeBehaviour>, sender: mpsc::Sender<Box<dyn Fn() + Send>>) {
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                let msg = format!("address: {:?}", address);
                let print_msg = move || {
                    debug!("{}", msg);
                };
                let _ = sender.send(Box::new(print_msg));
            }
            SwarmEvent::Behaviour(NodeBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                for peer in list {
                    let mut peer_id = peer.0;

                    let msg = format!("mDNS discovered a new peer: {peer_id}");
                    let print_msg = move || {
                        debug!("{}", msg);
                    };
                    let _ = sender.send(Box::new(print_msg));

                    // 测试发用文本
                    let req = { "Hi".to_string() };
                    swarm.behaviour_mut().handler.send_request(&mut peer_id, FileRequest(req));
                }
            }
            SwarmEvent::Behaviour(NodeBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                for peer in list {
                    let peer_id = peer.0;
                    let msg = format!("mDNS discover peer has expired: {peer_id}");
                    let print_msg = move || {
                        debug!("{}", msg);
                    };
                    let _ = sender.send(Box::new(print_msg));
                }
            }
            SwarmEvent::Behaviour(NodeBehaviourEvent::Handler(request_response::Event::Message { message, .. })) => match message {
                Message::Request { request, channel: _, .. } => {
                    let msg = format!("Request Message:{:#?}", request);
                    let print_msg = move || {
                        debug!("{}", msg);
                    };
                    let _ = sender.send(Box::new(print_msg));
                }
                Message::Response { request_id: _, response } => {
                    let msg = format!("Response Message:{:#?}", response);
                    let print_msg = move || {
                        debug!("{}", msg);
                    };
                    let _ = sender.send(Box::new(print_msg));
                }
            },
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                let msg = format!("ConnectionClosed peer_id {:#?}", peer_id.to_string());
                let print_msg = move || {
                    debug!("{}", msg);
                };
                let _ = sender.send(Box::new(print_msg));
            }
            _other => {
                let msg = format!("ConnectionClosed _other {:#?}", _other);
                let print_msg = move || {
                    debug!("{}", msg);
                };
                let _ = sender.send(Box::new(print_msg));
            }
        }
    }
}

pub fn run(sender: mpsc::Sender<Box<dyn Fn() + Send>>) -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut swarm = new_swarm();
    let multiaddrs = vec!["/ip4/0.0.0.0/tcp/0".parse()?, "/ip4/0.0.0.0/udp/0/quic-v1".parse()?];

    for addr in multiaddrs {
        swarm.listen_on(addr)?;
    }

    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(swarm_process(swarm, sender.clone()));
    });
    Ok(())
}
