use iced::{widget::Text, Font};

pub struct MonospaceText {}

impl MonospaceText {
    pub fn new<'a>(content: String) -> Text<'a> {
        Text::new(content).font(Font::MONOSPACE)
    }
}
pub fn monospace<'a>(content: &'a str) -> Text<'a> {
    Text::new(content).font(Font::MONOSPACE)
}
