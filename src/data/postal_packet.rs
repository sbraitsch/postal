use pcap::Packet;
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

#[derive(Debug, Clone)]
struct Header {
    timestamp: usize,
    cap_size: usize,
    pack_size: usize,
}

impl PostalPacket {
    pub fn from_packet(packet: Packet) -> Self {
        let source_ip;
        let destination_ip;
        if let [a, b, c, d] = packet.data[12..=15] {
            // ipheader bytes 12-15
            source_ip = Ipv4Addr::new(a, b, c, d);
        } else {
            panic!("Error parsing Source IP.")
        };
        if let [a, b, c, d] = packet.data[16..=19] {
            // ipheader bytes 16-19
            destination_ip = Ipv4Addr::new(a, b, c, d);
        } else {
            panic!("Error parsing Destination IP.")
        };
        Self {
            header: Header {
                timestamp: packet.header.ts.tv_sec as usize,
                cap_size: packet.header.caplen as usize,
                pack_size: packet.header.len as usize,
            },
            //data: packet.data.into_iter().copied().collect(),
            protocol: Protocol::from_u8(packet.data[23]), // eth header 14 byte, protocol in ipheader 9th byte -> 23
            port: (packet.data[36] as u16) << 8 | packet.data[37] as u16,
            source_ip,
            destination_ip,
            payload: String::new(), //String::from_utf8(packet.data[54..].to_vec()).unwrap(),
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
