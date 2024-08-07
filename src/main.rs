mod data;
mod elements;

use std::collections::HashMap;
use std::sync::Arc;

use data::packet_subscription::PacketSubscription;
use data::postal_options::PostalOptions;
use elements::monospace_text::monospace;
use elements::monospace_text::MonospaceText;
use elements::packet_list::PacketList;
use elements::sidebar::Sidebar;
use elements::styled_button::ButtonStyleSheet;
use iced::executor;
use iced::widget::scrollable;
use iced::widget::vertical_rule;
use iced::widget::Button;
use iced::widget::{button, column, container, horizontal_space, pick_list, row};
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use once_cell::sync::Lazy;

static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

#[tokio::main]
pub async fn main() -> iced::Result {
    Postal::run(Settings::default())
}

#[derive(Debug)]
struct Postal {
    sniff: bool,
    theme: Theme,
    packets: Vec<String>,
    options: HashMap<PostalOptions, bool>,
    sender: Arc<Mutex<Sender<String>>>,
    receiver: Arc<Mutex<Receiver<String>>>,
    task_handle: Option<JoinHandle<()>>,
    cancellation_token: CancellationToken,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(Theme),
    PacketsReceived(Vec<String>),
    Sniffing(bool),
    OptionChanged(PostalOptions, bool),
    Scrolled(scrollable::Viewport),
}

impl Application for Postal {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let (tx, rx) = mpsc::channel::<String>(1000);
        (
            Self {
                sniff: false,
                theme: Theme::GruvboxLight,
                packets: vec![],
                options: PostalOptions::as_map(),
                sender: Arc::new(Mutex::new(tx)),
                receiver: Arc::new(Mutex::new(rx)),
                task_handle: None,
                cancellation_token: CancellationToken::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Postal")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
            Message::PacketsReceived(mut packets) => {
                self.packets.append(&mut packets);
                if self.options[&PostalOptions::Autoscroll] {
                    return scrollable::snap_to(
                        SCROLLABLE_ID.clone(),
                        scrollable::RelativeOffset::END,
                    );
                }
            }
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
            Message::OptionChanged(f, b) => {
                self.options
                    .entry(f)
                    .and_modify(|toggled| *toggled = b)
                    .or_default();
            }
            Message::Scrolled(_) => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.sniff {
            iced_futures::Subscription::from_recipe(PacketSubscription::new(self.receiver.clone()))
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<Message> {
        let sniff_btn: Button<_> = if !self.sniff {
            button(monospace("Start Sniffing").size(20))
                .style(ButtonStyleSheet::new())
                .on_press(Message::Sniffing(!self.sniff))
        } else {
            button(monospace("Stop Sniffing").size(20))
                .style(ButtonStyleSheet::new())
                .on_press(Message::Sniffing(!self.sniff))
        };
        let footer = row![
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
            horizontal_space(),
            MonospaceText::new(format!("Captured {} Packets", self.packets.len())).size(20),
            horizontal_space(),
            sniff_btn,
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        let sidebar = Sidebar::view(&self.options);
        let packet_list = PacketList {}.view(&self.packets);
        let main = container(row![sidebar, vertical_rule(1), packet_list])
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Appearance::default().with_border(palette.background.strong.color, 2.0)
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
