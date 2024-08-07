use iced::widget::{container, scrollable, Column, Container};
use iced::{Alignment, Element, Length};

use crate::{Message, SCROLLABLE_ID};

use super::monospace_text::MonospaceText;

pub struct PacketList {}

impl PacketList {
    pub fn view(self, packets: &Vec<String>) -> Container<'static, Message> {
        let elem = packets
            .iter()
            .cloned()
            .map(|s| MonospaceText::new(s).into())
            .collect::<Vec<Element<Message>>>();

        container(
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
        .width(Length::FillPortion(5))
        .padding(10)
    }
}
