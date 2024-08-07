mod data;
mod elements;

use std::sync::Arc;

use data::packet_subscription::PacketSubscription;
use data::postal_packet::PostalPacket;
use elements::layout::Layout;
use iced::executor;
use iced::keyboard;
use iced::keyboard::key;
use iced::mouse;
use iced::widget::Button;
use iced::widget::{
    button, canvas, checkbox, column, container, horizontal_space, pick_list, row, text,
};
use iced::{
    color, Alignment, Application, Command, Element, Font, Length, Point, Rectangle, Renderer,
    Settings, Subscription, Theme,
};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

#[tokio::main]
pub async fn main() -> iced::Result {
    Postal::run(Settings::default())
}

#[derive(Debug)]
struct Postal {
    layout: Layout,
    explain: bool,
    sniff: bool,
    theme: Theme,
    packets: Vec<String>,
    sender: Arc<Mutex<Sender<String>>>,
    receiver: Arc<Mutex<Receiver<String>>>,
    task_handle: Option<JoinHandle<()>>,
    cancellation_token: CancellationToken,
}

#[derive(Debug, Clone)]
pub enum Message {
    ExplainToggled(bool),
    ThemeSelected(Theme),
    PacketReceived(PostalPacket),
    Sniffing(bool),
}

impl Application for Postal {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let (tx, rx) = mpsc::channel::<String>(100);

        (
            Self {
                layout: Layout::default(),
                explain: false,
                sniff: false,
                theme: Theme::GruvboxLight,
                packets: vec![],
                sender: Arc::new(Mutex::new(tx)),
                receiver: Arc::new(Mutex::new(rx)),
                task_handle: None,
                cancellation_token: CancellationToken::new(),
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
            Message::PacketReceived(p) => self.packets.push(p.to_string()),
            Message::Sniffing(sniff) => {
                self.sniff = sniff;
                if sniff {
                    println!("Started sniffing");
                    let tx = self.sender.clone();
                    let token = CancellationToken::new();
                    self.cancellation_token = token.clone();
                    self.task_handle = Some(tokio::spawn(async move {
                        PacketSubscription::sniff(tx, token).await
                    }));
                } else {
                    self.cancellation_token.cancel();
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.sniff {
            iced_futures::Subscription::from_recipe(PacketSubscription::new(self.receiver.clone()))
        } else {
            Subscription::none()
        }
        // use keyboard::key;
        // keyboard::on_key_release(|key, _modifiers| match key {
        //     keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::ExplainToggled(false)),
        //     keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::ExplainToggled(true)),
        //     _ => None,
        // })
        // Subscription::batch(vec![a, b])
    }

    fn view(&self) -> Element<Message> {
        let sniff_btn: Button<_> = if !self.sniff {
            button("Sniff").on_press(Message::Sniffing(!self.sniff))
        } else {
            button("Stop sniffing").on_press(Message::Sniffing(!self.sniff))
        };
        let footer = row![
            text(self.layout.title).size(20).font(Font::MONOSPACE),
            horizontal_space(),
            sniff_btn,
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
