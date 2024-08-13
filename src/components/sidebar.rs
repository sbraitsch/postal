use crate::{Element, Message, Postal};
use iced::widget::column;

use super::filters::Filters;
use super::settings::Settings;

pub struct Sidebar;

impl<'a> Sidebar {
    pub fn view(app: &'a Postal) -> Element<'a, Message> {
        column![Settings::view(app), Filters::view(app)].into()
    }
}
