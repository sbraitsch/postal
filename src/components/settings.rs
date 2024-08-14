use iced::{
    widget::{
        checkbox, column, container, horizontal_rule, horizontal_space, pick_list, row, Column,
        TextInput, Tooltip,
    },
    Alignment, Element, Font, Length, Theme,
};

use crate::{Message, Postal, NETWORK_INTERFACES};

use super::{
    monospace_text::{monospace, monospace_bold},
    solid_tooltip::SolidTooltip,
};

pub struct Settings;

impl<'a> Settings {
    pub fn view(app: &'a Postal) -> Element<Message> {
        let setting_header = container(
            monospace_bold("Settings")
                .size(20)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .width(Length::Fill)
        .padding(10);

        let interface_picker = pick_list(
            NETWORK_INTERFACES
                .iter()
                .map(|int| int.get_identifier().to_string())
                .collect::<Vec<String>>(),
            Some(app.network_interface.to_string()),
            Message::NetworkInterfaceSelected,
        )
        .font(Font::MONOSPACE)
        .width(Length::Fill);

        let window_input = TextInput::new("# of visible Packets", &app.cache_input)
            .on_input(Message::CacheInputChanged)
            .on_submit(Message::CacheSizeApplied)
            .font(Font::MONOSPACE)
            .padding(10);

        let opt_rows = app
            .options
            .iter()
            .map(|(&option, &(toggled, desc))| {
                let cb = checkbox("", toggled)
                    .font(Font::MONOSPACE)
                    .on_toggle(move |t| Message::OptionChanged(option, t));
                row![
                    Tooltip::new(
                        monospace(option.to_string()),
                        desc,
                        iced::widget::tooltip::Position::Right,
                    )
                    .padding(20)
                    .gap(20)
                    .style(SolidTooltip::new()),
                    horizontal_space(),
                    cb
                ]
                .into()
            })
            .collect::<Vec<_>>();

        let settings_container = container(
            Column::with_children(opt_rows)
                .spacing(10)
                .padding(10)
                .width(Length::Fill)
                .align_items(Alignment::Start)
                .push(column![monospace("View Limit:"), window_input])
                .push(column![monospace("Network Interface:"), interface_picker])
                .push(column![
                    monospace("Theme:"),
                    pick_list(Theme::ALL, Some(&app.theme), Message::ThemeSelected)
                        .font(Font::MONOSPACE)
                        .width(Length::Fill)
                ]),
        )
        .width(Length::FillPortion(1))
        .center_y();

        column![
            setting_header,
            horizontal_rule(1),
            settings_container,
            horizontal_rule(1),
        ]
        .into()
    }
}
