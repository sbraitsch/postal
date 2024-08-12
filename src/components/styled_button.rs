use iced::{widget::button, Border};

pub struct ButtonStyleSheet;

impl ButtonStyleSheet {
    pub fn new() -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(Self))
    }
}

impl button::StyleSheet for ButtonStyleSheet {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.palette().text,
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(style.palette().primary)),
            text_color: iced::Color::WHITE,
            border: Border::with_radius(3),
            ..Default::default()
        }
    }
}
