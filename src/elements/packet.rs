use iced::{
    widget::{container, scrollable, Column, Container},
    Alignment, Element, Length,
};

use crate::Message;

use super::monospace_text::MonospaceText;

pub struct PostalPacket {}

impl PostalPacket {
    pub fn view(packets: &Vec<String>) -> Container<'static, Message> {
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
            .height(Length::Fill),
        )
        .width(Length::FillPortion(3))
        .padding(10)
    }
}
