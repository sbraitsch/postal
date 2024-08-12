use crate::{Alignment, Element, Length, Message, Postal, NETWORK_INTERFACES};
use iced::widget::{
    checkbox, column, container, horizontal_rule, horizontal_space, pick_list, row, Column,
    TextInput,
};
use iced::{Font, Theme};

use super::monospace_text::{monospace, monospace_bold};

pub struct Sidebar;

impl<'a> Sidebar {
    pub fn view(app: &'a Postal) -> Element<'a, Message> {
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

        let interface_picker = pick_list(
            NETWORK_INTERFACES
                .iter()
                .map(|int| int.name.to_string())
                .collect::<Vec<String>>(),
            Some(app.network_interface.to_string()),
            Message::NetworkInterfaceSelected,
        )
        .font(Font::MONOSPACE)
        .width(Length::Fill);

        let opt_rows = app
            .options
            .iter()
            .map(|(&option, &toggled)| {
                let cb = checkbox("", toggled)
                    .font(Font::MONOSPACE)
                    .on_toggle(move |t| Message::OptionChanged(option, t));
                row![monospace(option.to_string()), horizontal_space(), cb].into()
            })
            .collect::<Vec<_>>();

        let settings_container = container(
            Column::with_children(opt_rows)
                .spacing(10)
                .padding(10)
                .width(Length::Fill)
                .align_items(Alignment::Start)
                .push(column![
                    monospace("Theme:"),
                    pick_list(Theme::ALL, Some(&app.theme), Message::ThemeSelected)
                        .font(Font::MONOSPACE)
                        .width(Length::Fill)
                ])
                .push(column![monospace("Network Interface:"), interface_picker]),
        )
        .width(Length::FillPortion(1))
        .center_y();

        let port_input = TextInput::new("e.g. 80, 443,..", &app.port_input)
            .on_input(Message::PortInputChanged)
            .on_submit(Message::PortFilterApplied)
            .font(Font::MONOSPACE)
            .padding(10);

        let type_rows = app
            .tp_types
            .iter()
            .map(|(filter, toggled)| {
                let f = filter.clone();
                let cb = checkbox("", *toggled)
                    .font(Font::MONOSPACE)
                    .on_toggle(move |t| Message::FilterChanged(f.clone(), t));
                row![monospace(filter.to_string()), horizontal_space(), cb].into()
            })
            .collect::<Vec<_>>();

        let types_container = container(
            Column::with_children(type_rows)
                .spacing(10)
                .padding(10)
                .width(Length::Fill)
                .align_items(Alignment::Start)
                .push(column![monospace("Ports:"), port_input]),
        )
        .height(Length::Fill)
        .width(Length::FillPortion(1));

        column![
            setting_header,
            horizontal_rule(1),
            settings_container,
            horizontal_rule(1),
            filter_header,
            horizontal_rule(1),
            types_container,
        ]
        .into()
    }
}
