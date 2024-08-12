use iced::{
    widget::{button, column, container, horizontal_space, row, vertical_rule, Button},
    Alignment, Element, Length, Theme,
};

use crate::{Message, Postal};

use super::{
    monospace_text::{monospace, monospace_bold},
    packet_list::PacketList,
    sidebar::Sidebar,
    styled_button::ButtonStyleSheet,
};

pub struct Layout {}

impl Layout {
    pub fn view(app: &Postal) -> Element<Message> {
        let sniff_btn: Button<_> = if !app.capturing {
            button(monospace_bold("Capture!").size(20))
                .style(ButtonStyleSheet::new())
                .on_press(Message::StartSniffing)
        } else {
            button(monospace_bold("Capturing..").size(20))
                .style(ButtonStyleSheet::new())
                .on_press(Message::StopSniffing)
        };
        let footer = row![
            button(monospace_bold("Clear").size(20))
                .style(ButtonStyleSheet::new())
                .on_press(Message::ClearCache),
            horizontal_space(),
            monospace(format!("Captured {} Packets", app.packets.len())).size(16),
            horizontal_space(),
            sniff_btn,
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        let sidebar = Sidebar::view(
            &app.options,
            &app.filter,
            &app.network_interface.name,
            &app.theme,
        );
        let packet_list = PacketList::view(&app.packets, &app.network_interface.ips, &app.filter);
        let main = container(row![sidebar, vertical_rule(1), packet_list])
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Appearance::default().with_border(palette.background.strong.color, 1.0)
            })
            .padding(4)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        column![main, footer].spacing(10).padding(10).into()
    }
}
