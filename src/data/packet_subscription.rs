use std::sync::Arc;

use iced::futures::stream;
use iced_futures::subscription::Recipe;
use pnet::{
    datalink::{self, Channel::Ethernet, Config},
    packet::{
        ethernet::EthernetPacket, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, tcp::TcpPacket,
        udp::UdpPacket, Packet,
    },
};
use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    },
    time::Duration,
};
use tokio_util::sync::CancellationToken;

use crate::Message;

#[derive(Debug)]
pub struct PacketSubscription {
    pub receiver: Arc<Mutex<Receiver<String>>>,
}

impl PacketSubscription {
    pub fn new(rx: Arc<Mutex<Receiver<String>>>) -> Self {
        PacketSubscription { receiver: rx }
    }

    pub async fn sniff(tx: Arc<Mutex<Sender<String>>>, token: CancellationToken) {
        let sender = tx.lock().await;
        let interface = datalink::interfaces()
            .into_iter()
            .find(|iface| iface.name == "en0")
            .expect("Interface not found");

        if let Ok(Ethernet(_, mut rx)) = datalink::channel(&interface, Config::default()) {
            while !token.is_cancelled() {
                if let Ok(packet) = rx.next() {
                    let eth_packet =
                        EthernetPacket::new(packet).expect("Failed to parse Ethernet packet");

                    // Check if it's an IPv4 packet
                    if eth_packet.get_ethertype() == pnet::packet::ethernet::EtherTypes::Ipv4 {
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        let ipv4_packet = Ipv4Packet::new(eth_packet.payload())
                            .expect("Failed to parse IPv4 packet");
                        println!("Captured IPv4 packet: {:?}", ipv4_packet);
                        match ipv4_packet.get_next_level_protocol() {
                            IpNextHeaderProtocols::Tcp => {
                                if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                                    let item = format!(
                                        "IPv4 Packet, Port: {}, Source IP: {}, Destination IP: {}",
                                        tcp_packet.get_destination(),
                                        ipv4_packet.get_source(),
                                        ipv4_packet.get_destination(),
                                    );
                                    let _ = sender.send(item).await;
                                }
                            }
                            IpNextHeaderProtocols::Udp => {
                                if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
                                    let item = format!(
                                        "IPv4 Packet, Port: {}, Source IP: {}, Destination IP: {}",
                                        udp_packet.get_destination(),
                                        ipv4_packet.get_source(),
                                        ipv4_packet.get_destination(),
                                    );
                                    let _ = sender.send(item).await;
                                }
                            }
                            _ => {
                                println!(
                                    "Other protocol: {}",
                                    ipv4_packet.get_next_level_protocol()
                                )
                            }
                        }
                    }
                }
            }
        }
        println!("Stopped sniffing task.")
    }
}

impl Recipe for PacketSubscription {
    type Output = Message;

    fn hash(&self, state: &mut iced_futures::core::Hasher) {
        use std::hash::Hash;
        "PacketSubscription".hash(state)
    }

    fn stream(
        self: Box<Self>,
        _input: iced_futures::subscription::EventStream,
    ) -> iced_futures::BoxStream<Self::Output> {
        Box::pin(stream::unfold(self.receiver.clone(), |r| async move {
            let mut rx = r.lock().await;
            match rx.recv().await {
                Some(s) => Some((Message::PacketReceived(s), r.clone())),
                None => None,
            }
        }))
    }
}
