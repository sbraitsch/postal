use core::str;

use iced::{
    widget::{row, Tooltip},
    Element, Length,
};
use pnet::packet::Packet;

use crate::{
    data::parsed_packet::{NetworkPacket, ParsedPacket, TransportPacket},
    utils::byte_formatter::format_size,
    Message,
};

use super::{
    colors::PostalColor,
    monospace_text::{monospace, monospace_bold},
    solid_tooltip::SolidTooltip,
};

impl ParsedPacket {
    pub fn view(&self, inbound: bool, relative_widths: &[u16]) -> Element<Message> {
        let size = self.data.len();
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

        let dir = if inbound {
            "Receiving".to_string()
        } else {
            "Sending".to_string()
        };
        let timestamp_text = monospace_bold(&self.timestring);
        let dir_text = monospace_bold(dir);
        let protocol_text = monospace_bold(protocol).style(PostalColor::MATTBLUE);
        let port_text = monospace_bold(format!(":{port}")).style(PostalColor::ORANGE);
        let source_text = monospace_bold(source).style(PostalColor::MINT);
        let destination_text = monospace_bold(dest).style(PostalColor::PURPLE);
        let size_text = monospace_bold(format_size(size));
        let inspect: Element<Message> = if port == 80 && !payload.is_empty() {
            Tooltip::new(
                monospace_bold("Inspect 💬").width(Length::FillPortion(relative_widths[7])),
                payload,
                iced::widget::tooltip::Position::Left,
            )
            .padding(20)
            .gap(20)
            .style(SolidTooltip::new())
            .into()
        } else {
            monospace("")
                .width(Length::FillPortion(relative_widths[7]))
                .into()
        };

        row![
            timestamp_text.width(Length::FillPortion(relative_widths[0])),
            dir_text.width(Length::FillPortion(relative_widths[1])),
            protocol_text.width(Length::FillPortion(relative_widths[2])),
            port_text.width(Length::FillPortion(relative_widths[3])),
            source_text.width(Length::FillPortion(relative_widths[4])),
            destination_text.width(Length::FillPortion(relative_widths[5])),
            size_text.width(Length::FillPortion(relative_widths[6])),
            inspect
        ]
        .width(Length::Fill)
        .into()
    }
}
