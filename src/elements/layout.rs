use iced::{theme, widget::scrollable};

use crate::{
    column, container, horizontal_space, row, square, Alignment, Element, Length, Message, Theme,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layout {
    pub title: &'static str,
    pub view: fn() -> Element<'static, Message>,
}

impl Layout {
    pub fn view(&self) -> Element<Message> {
        (self.view)()
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

fn layout_view<'a>() -> Element<'a, Message> {
    let header = container(
        row![
            square(40),
            horizontal_space(),
            "Header!",
            horizontal_space(),
            square(40),
        ]
        .padding(10)
        .align_items(Alignment::Center),
    )
    .style(|theme: &Theme| {
        let palette = theme.extended_palette();

        container::Appearance::default().with_border(palette.background.strong.color, 1)
    });

    let sidebar = container(
        column!["Sidebar!", square(50), square(50)]
            .spacing(40)
            .padding(10)
            .width(200)
            .align_items(Alignment::Center),
    )
    .style(theme::Container::Box)
    .height(Length::Fill)
    .center_y();

    let content = container(
        scrollable(
            column!["Content!", square(400), square(200), square(400), "The end"]
                .spacing(40)
                .align_items(Alignment::Center)
                .width(Length::Fill),
        )
        .height(Length::Fill),
    )
    .padding(10);

    column![header, row![sidebar, content]].into()
}
