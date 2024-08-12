use core::str;

use iced::{
    widget::{row, Tooltip},
    Element, Length,
};
use pnet::packet::Packet;

use crate::{
    data::parsed_packet::{NetworkPacket, ParsedPacket, TransportPacket},
    Message,
};

use super::{
    colors::PostalColor,
    monospace_text::{monospace, monospace_bold},
    solid_tooltip::SolidTooltip,
};

impl ParsedPacket {
    pub fn view(&self, inbound: bool) -> Element<Message> {
        let (port, protocol, payload) = match &self.transport {
            TransportPacket::Tcp(tcp) => (
                tcp.get_destination(),
                "TCP".to_string(),
                str::from_utf8(tcp.payload()).unwrap_or(""),
            ),
            TransportPacket::Udp(udp) => (
                udp.get_destination(),
                "UDP".to_string(),
                str::from_utf8(udp.payload()).unwrap_or(""),
            ),
            TransportPacket::Other => (0, "OTHER".to_string(), ""),
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

        let inspect: Element<Message> = if port == 80 && !payload.is_empty() {
            Tooltip::new(
                monospace_bold("[Inspect]").width(Length::FillPortion(1)),
                payload,
                iced::widget::tooltip::Position::Left,
            )
            .padding(20)
            .gap(20)
            .style(SolidTooltip::new())
            .into()
        } else {
            monospace("").width(Length::FillPortion(1)).into()
        };

        let dir = if inbound {
            "INCOMING".to_string()
        } else {
            "OUTGOING".to_string()
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
