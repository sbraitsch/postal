use iced::{
    widget::{
        checkbox, column, container, horizontal_rule, horizontal_space, row, Column, TextInput,
    },
    Alignment, Element, Font, Length,
};

use crate::{Message, Postal};

use super::monospace_text::{monospace, monospace_bold};

pub struct Filters;

impl<'a> Filters {
    pub fn view(app: &'a Postal) -> Element<Message> {
        let filter_header = container(
            monospace_bold("Protocols")
                .size(20)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .width(Length::Fill)
        .padding(10);

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

        column![filter_header, horizontal_rule(1), types_container,].into()
    }
}
