mod components;
mod data;
mod utils;

use std::collections::HashMap;
use std::sync::Arc;

use components::layout::Layout;
use data::packet_subscription::PacketSubscription;
use data::parsed_packet::ParsedPacket;
use data::parsed_packet::TransportPacket;
use data::postal_option::PostalOption;
use iced::executor;
use iced::widget::scrollable;
use iced::Size;
use iced::{Application, Command, Element, Settings, Subscription, Theme};
use pnet::datalink;
use pnet::datalink::NetworkInterface;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use once_cell::sync::Lazy;
use crate::data::os_network_interface::OSNetworkInterface;

static SCROLLABLE_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);
static NETWORK_INTERFACES: Lazy<Vec<OSNetworkInterface>> = Lazy::new(|| {
    datalink::interfaces()
        .into_iter()
        .filter(|e| !e.ips.is_empty())
        .map(|i| OSNetworkInterface::new(i))
        .collect()
});

#[tokio::main]
pub async fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = Size::new(1600.0, 900.0);
    Postal::run(settings)
}

#[derive(Debug)]
struct Postal {
    capturing: bool,
    theme: Theme,
    total_captured: usize,
    total_mem: usize,
    packets: Vec<ParsedPacket>,
    options: HashMap<PostalOption, (bool, &'static str)>,
    tp_types: HashMap<TransportPacket, bool>,
    port_input: String,
    port_list: Vec<u16>,
    cache_input: String,
    cache_size: usize,
    receiver: Option<Arc<Mutex<Receiver<ParsedPacket>>>>,
    cancellation_token: CancellationToken,
    network_interface: OSNetworkInterface,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(Theme),
    PacketsReceived(Vec<ParsedPacket>),
    PacketsDrained(Vec<ParsedPacket>),
    StartSniffing,
    StopSniffing,
    OptionChanged(PostalOption, bool),
    FilterChanged(TransportPacket, bool),
    Scrolled(scrollable::Viewport),
    NetworkInterfaceSelected(String),
    ClearCache,
    RowClicked(Vec<u8>),
    PortInputChanged(String),
    PortFilterApplied,
    CacheInputChanged(String),
    CacheSizeApplied,
}

impl Application for Postal {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                capturing: false,
                theme: Theme::Light,
                total_captured: 0,
                total_mem: 0,
                packets: Vec::with_capacity(1000),
                options: PostalOption::as_map(),
                tp_types: TransportPacket::as_map(),
                port_input: String::new(),
                port_list: vec![],
                cache_input: String::from("200"),
                cache_size: 200,
                receiver: None,
                cancellation_token: CancellationToken::new(),
                network_interface: NETWORK_INTERFACES
                    .iter()
                    .find(|i| i.interface.ips.iter().any(|ip| ip.is_ipv4() && !i.interface.is_loopback()))
                    .expect("No default network interface found.")
                    .clone()
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
                return append_new_packets(self, &mut packets);
            }
            Message::PacketsDrained(mut packets) => {
                self.capturing = false;
                return append_new_packets(self, &mut packets);
            }
            Message::StartSniffing => {
                println!("Capturing..");
                let (tx, rx) = mpsc::channel::<ParsedPacket>(1000);
                self.receiver = Some(Arc::new(Mutex::new(rx)));
                let token = CancellationToken::new();
                self.cancellation_token = token.clone();
                let ninf = self.network_interface.clone();
                let http_only = self.options[&PostalOption::HttpOnly].0;
                tokio::task::spawn_blocking(move || {
                    PacketSubscription::sniff(tx, ninf.interface, http_only, token)
                });
                self.capturing = true;
            }
            Message::StopSniffing => {
                println!("Capture stopped.");
                self.cancellation_token.cancel();
                self.capturing = false;
            }
            Message::OptionChanged(opt, b) => {
                self.options
                    .entry(opt)
                    .and_modify(|(toggled, _)| *toggled = b)
                    .or_default();
            }
            Message::Scrolled(_) => {}
            Message::NetworkInterfaceSelected(n) => {
                self.cancellation_token.cancel();
                self.network_interface = NETWORK_INTERFACES
                    .iter()
                    .find(|i| i.get_identifier() == &n)
                    .expect("Network Interface not recognized")
                    .clone();
                self.packets.clear();
            }
            Message::FilterChanged(f, b) => {
                self.tp_types
                    .entry(f)
                    .and_modify(|toggled| *toggled = b)
                    .or_default();
            }
            Message::ClearCache => {
                self.total_mem = 0;
                self.total_captured = 0;
                self.packets.clear();
            }
            Message::RowClicked(payload) => println!("{:?}", String::from_utf8(payload)),
            Message::PortInputChanged(ports) => self.port_input = ports,
            Message::PortFilterApplied => {
                self.port_list = self
                    .port_input
                    .split(",")
                    .filter_map(|port| {
                        let trimmed = port.trim();
                        match trimmed.parse::<u16>() {
                            Ok(val) => Some(val),
                            Err(_) => None,
                        }
                    })
                    .collect::<Vec<u16>>();
            }
            Message::CacheInputChanged(size) => self.cache_input = size,
            Message::CacheSizeApplied => {
                self.cache_size = self.cache_input.parse::<usize>().unwrap_or(1000)
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        Layout::view(self)
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.capturing {
            iced_futures::Subscription::from_recipe(PacketSubscription::new(
                self.receiver.as_ref().unwrap().clone(),
                self.cancellation_token.clone(),
            ))
        } else {
            Subscription::none()
        }
    }
}

fn append_new_packets(
    app: &mut Postal,
    new_packets: &mut Vec<ParsedPacket>,
) -> iced::Command<Message> {
    app.total_captured += new_packets.len();
    app.total_mem += new_packets.iter().fold(0, |acc, p| acc + p.data.len());
    app.packets.append(new_packets);
    Command::none()
    // See comment in postal_option.rs
    // if app.options[&PostalOption::Autoscroll].0 {
    //     return scrollable::snap_to(SCROLLABLE_ID.clone(), scrollable::RelativeOffset::END);
    // } else {
    //     Command::none()
    // }
}
