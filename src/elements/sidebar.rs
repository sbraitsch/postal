use crate::{
    container, container::Container, Alignment, Element, Length, Message, Renderer, Theme,
};
use iced::theme;
use iced::widget::{Column, Text};

pub struct Sidebar {
    categories: Vec<&'static str>,
}

impl Sidebar {
    pub fn view(self) -> Container<'static, Message> {
        let elem = self
            .categories
            .iter()
            .map(|&s| Text::from(s).into())
            .collect::<Vec<Element<Message>>>();

        container(
            Column::with_children(elem)
                .spacing(40)
                .padding(10)
                .width(200)
                .align_items(Alignment::Center),
        )
        .style(theme::Container::Box)
        .height(Length::Fill)
        .width(Length::FillPortion(1))
        .center_y()
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            categories: vec!["Some", "Types", "of", "Packages"],
        }
    }
}
