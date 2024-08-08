mod data;
mod elements;

use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

use data::packet_subscription::PacketSubscription;
use data::parsed_packet::ParsedPacket;
use data::parsed_packet::TransportPacket;
use data::postal_option::PostalOption;
use elements::monospace_text::monospace;
use elements::monospace_text::monospace_bold;
use elements::packet_list::PacketList;
use elements::sidebar::Sidebar;
use elements::styled_button::ButtonStyleSheet;
use iced::executor;
use iced::widget::scrollable;
use iced::widget::vertical_rule;
use iced::widget::Button;
use iced::widget::{button, column, container, horizontal_space, pick_list, row};
use iced::Font;
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};
use pnet::datalink;
use pnet::datalink::NetworkInterface;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use once_cell::sync::Lazy;

static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);
static NETWORK_INTERFACES: Lazy<Vec<NetworkInterface>> = Lazy::new(|| {
    datalink::interfaces()
        .iter()
        .cloned()
        .filter(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty())
        .collect()
});

#[tokio::main]
pub async fn main() -> iced::Result {
    Postal::run(Settings::default())
}

#[derive(Debug)]
struct Postal {
    sniff: bool,
    theme: Theme,
    packets: Vec<ParsedPacket>,
    options: HashMap<PostalOption, bool>,
    filter: HashMap<TransportPacket, bool>,
    sender: Arc<Mutex<Sender<ParsedPacket>>>,
    receiver: Arc<Mutex<Receiver<ParsedPacket>>>,
    cancellation_token: CancellationToken,
    network_interface: NetworkInterface,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(Theme),
    PacketsReceived(Vec<ParsedPacket>),
    Sniffing(bool),
    OptionChanged(PostalOption, bool),
    FilterChanged(TransportPacket, bool),
    Scrolled(scrollable::Viewport),
    NetworkInterfaceSelected(String),
}

impl Application for Postal {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let (tx, rx) = mpsc::channel::<ParsedPacket>(1000);
        (
            Self {
                sniff: false,
                theme: Theme::GruvboxDark,
                packets: vec![],
                options: PostalOption::as_map(),
                filter: TransportPacket::as_map(),
                sender: Arc::new(Mutex::new(tx)),
                receiver: Arc::new(Mutex::new(rx)),
                cancellation_token: CancellationToken::new(),
                network_interface: NETWORK_INTERFACES
                    .iter()
                    .find(|i| i.ips.iter().any(|ip| ip.is_ipv4() && !i.is_loopback()))
                    .expect("No default network interface found.")
                    .clone(),
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
                if self.options[&PostalOption::Autoscroll] {
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
                    let ninf = self.network_interface.clone();
                    thread::spawn(move || PacketSubscription::sniff(tx, ninf, token));
                } else {
                    self.cancellation_token.cancel();
                }
            }
            Message::OptionChanged(opt, b) => {
                self.options
                    .entry(opt)
                    .and_modify(|toggled| *toggled = b)
                    .or_default();
            }
            Message::Scrolled(_) => {}
            Message::NetworkInterfaceSelected(n) => {
                self.network_interface = NETWORK_INTERFACES
                    .iter()
                    .find(|i| i.name == n)
                    .expect("Network Interface not recognized")
                    .clone()
            }
            Message::FilterChanged(f, b) => {
                self.filter
                    .entry(f)
                    .and_modify(|toggled| *toggled = b)
                    .or_default();
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
    }

    fn view(&self) -> Element<Message> {
        let sniff_btn: Button<_> = if !self.sniff {
            button(monospace_bold("Capture    ").size(20))
                .style(ButtonStyleSheet::new())
                .on_press(Message::Sniffing(!self.sniff))
        } else {
            button(monospace_bold("Capturing..").size(20))
                .style(ButtonStyleSheet::new())
                .on_press(Message::Sniffing(!self.sniff))
        };
        let footer = row![
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected).font(Font::MONOSPACE),
            horizontal_space(),
            monospace(format!("Captured {} Packets", self.packets.len())).size(16),
            horizontal_space(),
            sniff_btn,
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        let sidebar = Sidebar::view(
            &self.options,
            &self.filter,
            self.network_interface.name.clone(),
        );
        let packet_list =
            PacketList {}.view(&self.packets, &self.network_interface.ips, &self.filter);
        let main = container(row![sidebar, vertical_rule(1), packet_list])
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Appearance::default().with_border(palette.background.strong.color, 1.0)
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
