use iced::{widget::row, Element, Length};

use crate::{
    data::parsed_packet::{ParsedPacket, TransportPacket},
    Message,
};

use super::{colors::PostalColor, monospace_text::monospace};

impl ParsedPacket {
    pub fn view(&self, inbound: bool) -> Option<Element<Message>> {
        let (port, protocol) = match &self.transport {
            TransportPacket::Tcp(tcp) => (format!(":{}", tcp.get_destination()), "TCP".to_string()),
            TransportPacket::Udp(udp) => (format!(":{}", udp.get_destination()), "UDP".to_string()),
            TransportPacket::Other => return None,
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
        let source_text = monospace(self.source_ip.to_string())
            .style(PostalColor::SOURCE)
            .width(Length::FillPortion(2));
        let destination_text = monospace(self.destination_ip.to_string())
            .style(PostalColor::DESTINATION)
            .width(Length::FillPortion(2));

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
