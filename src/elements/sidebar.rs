use std::collections::HashMap;

use crate::data::parsed_packet::TransportPacket;
use crate::data::postal_option::PostalOption;
use crate::{column, container, row, Alignment, Element, Length, Message, NETWORK_INTERFACES};
use iced::widget::{checkbox, horizontal_rule, horizontal_space, pick_list, Column};
use iced::Font;

use super::monospace_text::{monospace, MonospaceText};

pub struct Sidebar;

impl Sidebar {
    pub fn view(
        options: &HashMap<PostalOption, bool>,
        filters: &HashMap<TransportPacket, bool>,
        selected_interface: String,
    ) -> Element<'static, Message> {
        let header = container(
            monospace("Options")
                .size(20)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .width(Length::Fill);

        let mut opt_rows = options
            .iter()
            .map(|(&option, &toggled)| {
                let opt = option.clone();
                let cb = checkbox("", toggled)
                    .font(Font::MONOSPACE)
                    .on_toggle(move |t| Message::OptionChanged(opt.clone(), t));
                row![
                    MonospaceText::new(option.to_string()),
                    horizontal_space(),
                    cb
                ]
                .into()
            })
            .collect::<Vec<_>>();

        let filter_rows = filters.clone().into_iter().map(|(filter, toggled)| {
            let filt = filter.clone();
            let cb = checkbox("", toggled)
                .font(Font::MONOSPACE)
                .on_toggle(move |t| Message::FilterChanged(filt.clone(), t));
            row![
                MonospaceText::new(filter.to_string()),
                horizontal_space(),
                cb
            ]
            .into()
        });

        opt_rows.extend(filter_rows);

        let settings = container(
            Column::with_children(opt_rows)
                .spacing(10)
                .padding(10)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_items(Alignment::Start),
        )
        .height(Length::Fill)
        .width(Length::FillPortion(1))
        .center_y();

        let interface_picker = pick_list(
            NETWORK_INTERFACES
                .iter()
                .map(|int| int.name.to_string())
                .collect::<Vec<String>>(),
            Some(selected_interface),
            Message::NetworkInterfaceSelected,
        )
        .font(Font::MONOSPACE)
        .width(Length::Fill);

        column![header, horizontal_rule(5), settings, interface_picker].into()
    }
}
