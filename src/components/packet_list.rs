use std::collections::HashMap;

use iced::widget::{column, container, horizontal_rule, row, scrollable, Column};
use iced::{Alignment, Element, Length};
use pnet::ipnetwork::IpNetwork;

use crate::data::parsed_packet::{ParsedPacket, TransportPacket};
use crate::{Message, SCROLLABLE_ID};

use super::monospace_text::monospace_bold;

pub struct PacketList {}

impl PacketList {
    pub fn view<'a>(
        packets: &'a Vec<ParsedPacket>,
        own_ips: &'a Vec<IpNetwork>,
        filter: &'a HashMap<TransportPacket, bool>,
    ) -> Element<'a, Message> {
        let header = row![
            monospace_bold("Direction")
                .size(16)
                .width(Length::FillPortion(1)),
            monospace_bold("Protocol")
                .size(16)
                .width(Length::FillPortion(1)),
            monospace_bold("Port")
                .size(16)
                .width(Length::FillPortion(1)),
            monospace_bold("Source IP")
                .size(16)
                .width(Length::FillPortion(3)),
            monospace_bold("Destination IP")
                .size(16)
                .width(Length::FillPortion(3)),
        ]
        .width(Length::Fill)
        .padding(10);

        let elem = packets
            .iter()
            .filter(|p| filter[&p.transport])
            .filter_map(|p| {
                p.view(
                    own_ips
                        .iter()
                        .any(|nw| nw.ip() == p.get_source_ip().unwrap()),
                )
            })
            .collect::<Vec<Element<Message>>>();

        let packet_list = container(
            scrollable(
                Column::with_children(elem)
                    .spacing(10)
                    .align_items(Alignment::Start)
                    .width(Length::Fill),
            )
            .id(SCROLLABLE_ID.clone())
            .on_scroll(Message::Scrolled)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .padding(10);

        column![header, horizontal_rule(1), packet_list]
            .width(Length::FillPortion(5))
            .into()
    }
}
