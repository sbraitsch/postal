use std::collections::HashMap;

use crate::data::parsed_packet::TransportPacket;
use crate::data::postal_option::PostalOption;
use crate::{column, container, row, Alignment, Element, Length, Message, NETWORK_INTERFACES};
use iced::widget::{checkbox, horizontal_rule, horizontal_space, pick_list, Column};
use iced::{Font, Theme};

use super::monospace_text::{monospace, monospace_bold};

pub struct Sidebar;

impl<'a> Sidebar {
    pub fn view(
        options: &HashMap<PostalOption, bool>,
        filters: &'a HashMap<TransportPacket, bool>,
        selected_interface: &'a String,
        selected_theme: &'a Theme,
    ) -> Element<'a, Message> {
        let setting_header = container(
            monospace_bold("Settings")
                .size(20)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .width(Length::Fill)
        .padding(10);

        let filter_header = container(
            monospace_bold("Protocols")
                .size(20)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .width(Length::Fill)
        .padding(10);

        let opt_rows = options
            .iter()
            .map(|(&option, &toggled)| {
                let cb = checkbox("", toggled)
                    .font(Font::MONOSPACE)
                    .on_toggle(move |t| Message::OptionChanged(option, t));
                row![monospace(option.to_string()), horizontal_space(), cb].into()
            })
            .collect::<Vec<_>>();

        let filter_rows = filters
            .into_iter()
            .map(|(filter, toggled)| {
                let f = filter.clone();
                let cb = checkbox("", *toggled)
                    .font(Font::MONOSPACE)
                    .on_toggle(move |t| Message::FilterChanged(f.clone(), t));
                row![monospace(filter.to_string()), horizontal_space(), cb].into()
            })
            .collect::<Vec<_>>();
        let settings_container = container(
            Column::with_children(opt_rows)
                .spacing(10)
                .padding(10)
                .width(Length::Fill)
                .align_items(Alignment::Start)
                .push(
                    pick_list(Theme::ALL, Some(selected_theme), Message::ThemeSelected)
                        .font(Font::MONOSPACE),
                ),
        )
        .width(Length::FillPortion(1))
        .center_y();

        let filters_container = container(
            Column::with_children(filter_rows)
                .spacing(10)
                .padding(10)
                .width(Length::Fill)
                .align_items(Alignment::Start),
        )
        .height(Length::Fill)
        .width(Length::FillPortion(1));

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

        column![
            setting_header,
            horizontal_rule(1),
            settings_container,
            horizontal_rule(1),
            filter_header,
            horizontal_rule(1),
            filters_container,
            horizontal_rule(1),
            interface_picker
        ]
        .into()
    }
}
