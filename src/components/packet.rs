use iced::{
    widget::{row, Button},
    Element, Length,
};
use pnet::packet::Packet;

use crate::{
    data::parsed_packet::{NetworkPacket, ParsedPacket, TransportPacket},
    Message,
};

use super::{
    colors::PostalColor,
    monospace_text::monospace,
    styled_buttons::{PayloadButton, SubtleButton},
};

impl ParsedPacket {
    pub fn view(&self, inbound: bool) -> Element<Message> {
        let (port, protocol, payload) = match &self.transport {
            TransportPacket::Tcp(tcp) => (
                tcp.get_destination(),
                "TCP".to_string(),
                tcp.payload().to_vec(),
            ),
            TransportPacket::Udp(udp) => (
                udp.get_destination(),
                "UDP".to_string(),
                udp.payload().to_vec(),
            ),
            TransportPacket::Other => (0, "OTHER".to_string(), vec![]),
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

        let inspect = if port == 80 {
            Button::new("Inspect")
                .on_press(Message::RowClicked(payload))
                .style(PayloadButton::new())
                .width(Length::FillPortion(1))
        } else {
            Button::new("Encrypted")
                .style(SubtleButton::new())
                .width(Length::FillPortion(1))
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
        let port_text = monospace(format!(":{port}"))
            .style(PostalColor::PORT)
            .width(Length::FillPortion(1));
        let source_text = monospace(source)
            .style(PostalColor::SOURCE)
            .width(Length::FillPortion(3));
        let destination_text = monospace(dest)
            .style(PostalColor::DESTINATION)
            .width(Length::FillPortion(3));

        row![
            dir_text,
            protocol_text,
            port_text,
            source_text,
            destination_text,
            inspect
        ]
        .width(Length::Fill)
        .into()
    }
}
