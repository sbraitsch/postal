use core::str;
use std::hash::{Hash, Hasher};
use std::{collections::HashMap, net::Ipv4Addr};

use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    tcp::TcpPacket,
    udp::UdpPacket,
    Packet,
};

#[derive(Debug, Clone)]
pub struct ParsedPacket {
    pub source_ip: Ipv4Addr,
    pub destination_ip: Ipv4Addr,
    pub transport: TransportPacket,
}

#[derive(Debug)]
pub enum TransportPacket {
    Tcp(TcpPacket<'static>),
    Udp(UdpPacket<'static>),
    Other,
}

impl ParsedPacket {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if let Some(eth_packet) = EthernetPacket::new(data) {
            if eth_packet.get_ethertype() == EtherTypes::Ipv4 {
                if let Some(ipv4_packet) = Ipv4Packet::new(eth_packet.payload()) {
                    let transport = match ipv4_packet.get_next_level_protocol() {
                        IpNextHeaderProtocols::Tcp => TransportPacket::Tcp(
                            TcpPacket::owned(ipv4_packet.payload().to_vec()).unwrap(),
                        ),
                        IpNextHeaderProtocols::Udp => TransportPacket::Udp(
                            UdpPacket::owned(ipv4_packet.payload().to_vec()).unwrap(),
                        ),
                        _ => TransportPacket::Other,
                    };
                    return Some(Self {
                        source_ip: ipv4_packet.get_source(),
                        destination_ip: ipv4_packet.get_destination(),
                        transport,
                    });
                }
            }
        }
        None
    }
}

impl ToString for ParsedPacket {
    fn to_string(&self) -> String {
        match &self.transport {
            TransportPacket::Tcp(tcp) => {
                format!(
                    "TCP Packet @ Port: {}, Source IP: {}, Destination IP: {}\n{}",
                    tcp.get_destination(),
                    self.source_ip,
                    self.destination_ip,
                    str::from_utf8(tcp.payload()).unwrap_or("")
                )
            }
            TransportPacket::Udp(udp) => format!(
                "UDP Packet @ Port: {}, Source IP: {}, Destination IP: {}\n{}",
                udp.get_destination(),
                self.source_ip,
                self.destination_ip,
                str::from_utf8(udp.payload()).unwrap_or("")
            ),
            TransportPacket::Other => format!("Unsupported Packet Type"),
        }
    }
}

impl TransportPacket {
    pub fn as_map() -> HashMap<TransportPacket, bool> {
        let mut map = HashMap::new();
        map.insert(
            TransportPacket::Tcp(TcpPacket::owned(vec![0u8; 20]).unwrap()),
            true,
        );
        map.insert(
            TransportPacket::Udp(UdpPacket::owned(vec![0u8; 20]).unwrap()),
            false,
        );
        map
    }
}

impl PartialEq for TransportPacket {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for TransportPacket {}

impl Hash for TransportPacket {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl Clone for TransportPacket {
    fn clone(&self) -> Self {
        match self {
            TransportPacket::Tcp(ref tcp) => {
                TransportPacket::Tcp(TcpPacket::owned(tcp.packet().to_vec()).unwrap())
            }
            TransportPacket::Udp(ref udp) => {
                TransportPacket::Udp(UdpPacket::owned(udp.packet().to_vec()).unwrap())
            }
            TransportPacket::Other => TransportPacket::Other,
        }
    }
}

impl ToString for TransportPacket {
    fn to_string(&self) -> String {
        match self {
            TransportPacket::Tcp(_) => "TCP".to_string(),
            TransportPacket::Udp(_) => "UDP".to_string(),
            TransportPacket::Other => "OTHER".to_string(),
        }
    }
}
