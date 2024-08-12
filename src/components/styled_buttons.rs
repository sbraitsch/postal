use iced::{widget::button, Border};

pub struct SubtleButton;

impl SubtleButton {
    pub fn new() -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(Self))
    }
}

impl button::StyleSheet for SubtleButton {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.palette().text,
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(style.palette().text)),
            text_color: style.palette().background,
            border: Border::with_radius(3),
            ..Default::default()
        }
    }
}

pub struct PayloadButton;

impl PayloadButton {
    pub fn new() -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(Self))
    }
}

impl button::StyleSheet for PayloadButton {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.palette().background,
            background: Some(iced::Background::Color(style.palette().text)),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(style.palette().text)),
            text_color: style.palette().background,
            border: Border::with_radius(3),
            ..Default::default()
        }
    }
}
