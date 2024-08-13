use iced::widget::{column, container, horizontal_rule, row, scrollable, Column};
use iced::{Alignment, Element, Length};

use crate::{Message, Postal, SCROLLABLE_ID};

use super::monospace_text::monospace_bold;

pub struct PacketList {}

impl PacketList {
    pub fn view<'a>(app: &'a Postal) -> Element<'a, Message> {
        let relative_widths = [1, 1, 1, 3, 3, 1, 1];
        let header = row![
            monospace_bold("Direction")
                .size(16)
                .width(Length::FillPortion(relative_widths[0])),
            monospace_bold("Protocol")
                .size(16)
                .width(Length::FillPortion(relative_widths[1])),
            monospace_bold("Port")
                .size(16)
                .width(Length::FillPortion(relative_widths[2])),
            monospace_bold("Source IP")
                .size(16)
                .width(Length::FillPortion(relative_widths[3])),
            monospace_bold("Destination IP")
                .size(16)
                .width(Length::FillPortion(relative_widths[4])),
            monospace_bold("Size")
                .size(16)
                .width(Length::FillPortion(relative_widths[5])),
            monospace_bold("Payload")
                .size(16)
                .width(Length::FillPortion(relative_widths[6])),
        ]
        .width(Length::Fill)
        .padding(10);

        let elem = app
            .packets
            .iter()
            .filter(|p| {
                app.tp_types[&p.transport]
                    && match p.get_port() {
                        Some(port) => app.port_list.contains(&port) || app.port_list.is_empty(),
                        None => false,
                    }
            })
            .map(|p| {
                p.view(
                    app.network_interface
                        .ips
                        .iter()
                        .any(|nw| nw.ip() == p.get_source_ip().unwrap()),
                    &relative_widths,
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
