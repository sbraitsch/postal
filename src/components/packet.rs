use iced::{widget::row, Element, Length};

use crate::{
    data::parsed_packet::{NetworkPacket, ParsedPacket, TransportPacket},
    Message,
};

use super::{colors::PostalColor, monospace_text::monospace};

impl ParsedPacket {
    pub fn view(&self, inbound: bool) -> Option<Element<Message>> {
        let (port, protocol) = match &self.transport {
            TransportPacket::Tcp(tcp) => (format!(":{}", tcp.get_destination()), "TCP".to_string()),
            TransportPacket::Udp(udp) => (format!(":{}", udp.get_destination()), "UDP".to_string()),
            TransportPacket::Other => (String::new(), "OTHER".to_string()),
        };
        let (source, dest) = match &self.net {
            NetworkPacket::Ipv4(v4) => (
                v4.get_source().to_string(),
                v4.get_destination().to_string(),
            ),
            NetworkPacket::Ipv6(v6) => (
                v6.get_source().to_string(),
                v6.get_destination().to_string(),
            ),
            NetworkPacket::Other => (String::new(), String::new()),
        };

        let dir = if inbound {
            "IN  <-".to_string()
        } else {
            "OUT ->".to_string()
        };
        let dir_text = monospace(dir)
            .style(PostalColor::DIRECTION)
            .width(Length::FillPortion(1));
        let protocol_text = monospace(protocol)
            .style(PostalColor::PROTOCOL)
            .width(Length::FillPortion(1));
        let port_text = monospace(port)
            .style(PostalColor::PORT)
            .width(Length::FillPortion(1));

        let source_text = monospace(source)
            .style(PostalColor::SOURCE)
            .width(Length::FillPortion(3));

        let destination_text = monospace(dest)
            .style(PostalColor::DESTINATION)
            .width(Length::FillPortion(3));

        Some(
            row![
                dir_text,
                protocol_text,
                port_text,
                source_text,
                destination_text
            ]
            .width(Length::Fill)
            .into(),
        )
    }
}
