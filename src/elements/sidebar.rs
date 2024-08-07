use std::collections::HashMap;

use crate::data::postal_options::PostalOptions;
use crate::{column, container, row, Alignment, Element, Length, Message};
use iced::widget::{checkbox, horizontal_rule, horizontal_space, Column};
use iced::Font;

use super::monospace_text::{monospace, MonospaceText};

pub struct Sidebar;

impl Sidebar {
    pub fn view(filters: &HashMap<PostalOptions, bool>) -> Element<'static, Message> {
        let header = container(
            monospace("Options")
                .size(20)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .width(Length::Fill);

        let elem = filters
            .iter()
            .map(|(&filter, &toggled)| {
                let f = filter.clone();
                let cb = checkbox("", toggled)
                    .font(Font::MONOSPACE)
                    .on_toggle(move |t| Message::OptionChanged(f, t));
                container(row![
                    MonospaceText::new(filter.to_string()),
                    horizontal_space(),
                    cb
                ])
                .into()
            })
            .collect::<Vec<_>>();

        let filters = container(
            Column::with_children(elem)
                .spacing(10)
                .padding(10)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_items(Alignment::Start),
        )
        .height(Length::Fill)
        .width(Length::FillPortion(1))
        .center_y();

        column![header, horizontal_rule(5), filters].into()
    }
}
