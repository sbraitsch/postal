use crate::{container, container::Container, Alignment, Element, Length, Message};
use iced::widget::{scrollable, Column, Text};

pub struct PacketList {}

impl PacketList {
    pub fn view(self, packets: &Vec<String>) -> Container<'static, Message> {
        let elem = packets
            .iter()
            .cloned()
            .map(|s| Text::new(s).into())
            .collect::<Vec<Element<Message>>>();

        container(
            scrollable(
                Column::with_children(elem)
                    .spacing(20)
                    .align_items(Alignment::Start)
                    .width(Length::Fill),
            )
            .height(Length::Fill),
        )
        .width(Length::FillPortion(3))
        .padding(10)
    }
}
