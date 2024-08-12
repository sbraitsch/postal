use iced::widget::container;

pub struct ColoredContainer;

impl ColoredContainer {
    pub fn new() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(Self))
    }
}

impl container::StyleSheet for ColoredContainer {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(style.palette().text)),
            text_color: Some(style.palette().background),
            ..Default::default()
        }
    }
}
