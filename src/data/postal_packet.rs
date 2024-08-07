use iced::futures::TryStream;
use std::{fmt, net::Ipv4Addr};

use super::protocol::Protocol;

#[derive(Debug, Clone)]
pub struct PostalPacket {
    header: Header,
    //data: Vec<u8>,
    protocol: Protocol,
    pub port: u16,
    source_ip: Ipv4Addr,
    destination_ip: Ipv4Addr,
    payload: String,
}

#[derive(Debug, Clone, Default)]
struct Header {
    timestamp: usize,
    cap_size: usize,
    pack_size: usize,
}

impl PostalPacket {
    pub fn new(packet: String) -> Self {
        PostalPacket {
            header: Header {
                ..Default::default()
            },
            protocol: Protocol::Tcp,
            port: Default::default(),
            source_ip: Ipv4Addr::new(1, 2, 3, 4),
            destination_ip: Ipv4Addr::new(1, 2, 3, 4),
            payload: packet,
        }
    }
}

impl fmt::Display for PostalPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Protocol: {}, Port: {}, Source IP: {}, Destination IP: {}, Payload: {}",
            self.protocol, self.port, self.source_ip, self.destination_ip, self.payload
        )
    }
}
