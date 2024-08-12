use iced::{border::Radius, widget::container, Border};

pub struct SolidTooltip;

impl SolidTooltip {
    pub fn new() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(Self))
    }
}

impl container::StyleSheet for SolidTooltip {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(style.palette().background)),
            text_color: Some(style.palette().text),
            border: Border {
                color: style.palette().text,
                width: 2.0,
                radius: Radius::from(3),
            },
            ..Default::default()
        }
    }
}
