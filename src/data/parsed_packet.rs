use core::fmt;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::net::IpAddr;

use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    tcp::TcpPacket,
    udp::UdpPacket,
    Packet,
};

#[derive(Debug)]
pub struct ParsedPacket {
    _data: Vec<u8>,
    _eth: EthernetPacket<'static>,
    pub net: NetworkPacket,
    pub transport: TransportPacket,
}

impl ParsedPacket {
    const ETHERNET_HEADER: usize = 14;

    pub fn parse(data: Vec<u8>) -> Option<Self> {
        let raw_data_static: &'static [u8] = unsafe { std::mem::transmute(&data[..]) };
        let eth = EthernetPacket::new(raw_data_static)?;
        let net = match eth.get_ethertype() {
            EtherTypes::Ipv4 => NetworkPacket::Ipv4(Ipv4Packet::new(&raw_data_static[14..])?),
            EtherTypes::Ipv6 => NetworkPacket::Ipv6(Ipv6Packet::new(&raw_data_static[14..])?),
            _ => return None,
        };

        let transport = match net {
            NetworkPacket::Ipv4(ref p) => {
                let offset = Self::ETHERNET_HEADER + p.get_header_length() as usize * 4;
                parse_transport_protocol(p.get_next_level_protocol(), raw_data_static, offset)?
            }
            NetworkPacket::Ipv6(ref p) => {
                let header_len = p.packet().len() - p.get_payload_length() as usize;
                let offset = Self::ETHERNET_HEADER + header_len;
                parse_transport_protocol(p.get_next_header(), raw_data_static, offset)?
            }
            NetworkPacket::Other => return None,
        };

        Some(Self {
            _data: data,
            _eth: eth,
            net,
            transport,
        })
    }

    pub fn get_source_ip(&self) -> Option<IpAddr> {
        match &self.net {
            NetworkPacket::Ipv4(v4) => Some(IpAddr::V4(v4.get_source())),
            NetworkPacket::Ipv6(v6) => Some(IpAddr::V6(v6.get_source())),
            NetworkPacket::Other => None,
        }
    }
}

fn parse_transport_protocol(
    protocol: IpNextHeaderProtocol,
    data: &'static [u8],
    offset: usize,
) -> Option<TransportPacket> {
    return Some(match protocol {
        IpNextHeaderProtocols::Tcp => {
            TransportPacket::Tcp(TcpPacket::new(&data[offset..]).unwrap())
        }
        IpNextHeaderProtocols::Udp => {
            TransportPacket::Udp(UdpPacket::new(&data[offset..]).unwrap())
        }
        _ => TransportPacket::Other,
    });
}

impl fmt::Display for ParsedPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (source, dest) = match &self.net {
            NetworkPacket::Ipv4(net) => (
                IpAddr::V4(net.get_source()),
                IpAddr::V4(net.get_destination()),
            ),
            NetworkPacket::Ipv6(net) => (
                IpAddr::V6(net.get_source()),
                IpAddr::V6(net.get_destination()),
            ),
            NetworkPacket::Other => return write!(f, ""),
        };

        let (protocol, port) = match &self.transport {
            TransportPacket::Tcp(tcp) => ("TCP", tcp.get_destination()),
            TransportPacket::Udp(udp) => ("UDP", udp.get_destination()),
            TransportPacket::Other => return write!(f, "Unsupported Transport Protocol"),
        };

        write!(
            f,
            "{} Packet @ Port: {}, Source IP: {}, Destination IP: {}",
            protocol, port, source, dest
        )
    }
}

impl Clone for ParsedPacket {
    fn clone(&self) -> Self {
        let eth_data = self._eth.packet().to_vec();
        let eth_clone = EthernetPacket::owned(eth_data).unwrap();
        Self {
            _data: self._data.clone(),
            _eth: eth_clone,
            net: self.net.clone(),
            transport: self.transport.clone(),
        }
    }
}

#[derive(Debug)]
pub enum TransportPacket {
    Tcp(TcpPacket<'static>),
    Udp(UdpPacket<'static>),
    Other,
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
        map.insert(TransportPacket::Other, false);
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

#[derive(Debug)]
pub enum NetworkPacket {
    Ipv4(Ipv4Packet<'static>),
    Ipv6(Ipv6Packet<'static>),
    Other,
}

impl NetworkPacket {
    pub fn as_map() -> HashMap<NetworkPacket, bool> {
        let mut map = HashMap::new();
        map.insert(
            NetworkPacket::Ipv4(Ipv4Packet::owned(vec![0u8; 20]).unwrap()),
            true,
        );
        map.insert(
            NetworkPacket::Ipv6(Ipv6Packet::owned(vec![0u8; 40]).unwrap()),
            false,
        );
        map.insert(NetworkPacket::Other, false);
        map
    }
}

impl PartialEq for NetworkPacket {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for NetworkPacket {}

impl Hash for NetworkPacket {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl Clone for NetworkPacket {
    fn clone(&self) -> Self {
        match self {
            NetworkPacket::Ipv4(ref ip) => {
                NetworkPacket::Ipv4(Ipv4Packet::owned(ip.packet().to_vec()).unwrap())
            }
            NetworkPacket::Ipv6(ref ip) => {
                NetworkPacket::Ipv6(Ipv6Packet::owned(ip.packet().to_vec()).unwrap())
            }
            NetworkPacket::Other => NetworkPacket::Other,
        }
    }
}

impl ToString for NetworkPacket {
    fn to_string(&self) -> String {
        match self {
            NetworkPacket::Ipv4(_) => "IPv4".to_string(),
            NetworkPacket::Ipv6(_) => "IPv6".to_string(),
            NetworkPacket::Other => "OTHER".to_string(),
        }
    }
}
