use std::borrow::Cow;

use iced::widget::Text;

pub struct CustomFont;

impl CustomFont {
    pub const BOLD: iced::Font = iced::Font {
        family: iced::font::Family::Monospace,
        weight: iced::font::Weight::Bold,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    };
}

pub fn monospace<'a>(content: impl Into<Cow<'a, str>>) -> Text<'a> {
    Text::new(content) //.font(Font::MONOSPACE)
}

pub fn monospace_bold<'a>(content: impl Into<Cow<'a, str>>) -> Text<'a> {
    Text::new(content) //.font(CustomFont::BOLD)
}
