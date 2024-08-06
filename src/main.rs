mod data;
mod elements;

use data::packet_subscription;
use data::postal_packet::PostalPacket;
use elements::layout::Layout;
use iced::executor;
use iced::keyboard;
use iced::mouse;
use iced::widget::{canvas, checkbox, column, container, horizontal_space, pick_list, row, text};
use iced::{
    color, Alignment, Application, Command, Element, Font, Length, Point, Rectangle, Renderer,
    Settings, Subscription, Theme,
};
use pcap::Packet;

pub fn main() -> iced::Result {
    Postal::run(Settings::default())
}

#[derive(Debug)]
struct Postal {
    layout: Layout,
    explain: bool,
    theme: Theme,
    packets: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    ExplainToggled(bool),
    ThemeSelected(Theme),
    PacketReceived(PostalPacket),
}

impl Application for Postal {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                layout: Layout::default(),
                explain: false,
                theme: Theme::GruvboxLight,
                packets: vec![],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        self.layout.title.to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::ExplainToggled(explain) => {
                self.explain = explain;
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
            Message::PacketReceived(p) => {
                if self.packets.len() < 20 {
                    self.packets.push(p.to_string())
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        packet_subscription::pcap_subscribe().map(Message::PacketReceived)
        // use keyboard::key;

        // keyboard::on_key_release(|key, _modifiers| match key {
        //     keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::ExplainToggled(false)),
        //     keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::ExplainToggled(true)),
        //     _ => None,
        // })
    }

    fn view(&self) -> Element<Message> {
        let footer = row![
            text(self.layout.title).size(20).font(Font::MONOSPACE),
            horizontal_space(),
            checkbox("Explain", self.explain).on_toggle(Message::ExplainToggled),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        let main = container(if self.explain {
            self.layout.view(&self.packets).explain(color!(0x0000ff))
        } else {
            self.layout.view(&self.packets)
        })
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();

            container::Appearance::default().with_border(palette.background.strong.color, 4.0)
        })
        .padding(4)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

        column![main, footer].spacing(10).padding(10).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn centered<'a>() -> Element<'a, Message> {
    container(text("I am centered!").size(50))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}

fn column_<'a>() -> Element<'a, Message> {
    column![
        "A column can be used to",
        "lay out widgets vertically.",
        square(50),
        square(50),
        square(50),
        "The amount of space between",
        "elements can be configured!",
    ]
    .spacing(40)
    .into()
}

fn row_<'a>() -> Element<'a, Message> {
    row![
        "A row works like a column...",
        square(50),
        square(50),
        square(50),
        "but lays out widgets horizontally!",
    ]
    .spacing(40)
    .into()
}

fn space<'a>() -> Element<'a, Message> {
    row!["Left!", horizontal_space(), "Right!"].into()
}

fn square<'a>(size: impl Into<Length> + Copy) -> Element<'a, Message> {
    struct Square;

    impl canvas::Program<Message> for Square {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());

            let palette = theme.extended_palette();

            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette.background.strong.color,
            );

            vec![frame.into_geometry()]
        }
    }

    canvas(Square).width(size).height(size).into()
}
