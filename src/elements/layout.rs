use iced::{widget::scrollable, widget::Text};
use pcap::Packet;

use crate::{column, container, row, square, Alignment, Element, Length, Message};

use super::{packet_list::PacketList, sidebar::Sidebar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layout {
    pub title: &'static str,
    pub view: fn(&Vec<String>) -> Element<'static, Message>,
}

impl Layout {
    pub fn view(&self, packets: &Vec<String>) -> Element<Message> {
        (self.view)(packets)
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            title: "Postal",
            view: layout_view,
        }
    }
}

fn layout_view<'a>(packets: &Vec<String>) -> Element<'a, Message> {
    let sidebar = Sidebar::default().view();

    let packet_list = PacketList {}.view(packets);

    row![sidebar, packet_list].into()
}
