use std::sync::Arc;

use iced::futures::stream;
use iced_futures::subscription::Recipe;
use pnet::datalink::{self, Channel::Ethernet, Config, NetworkInterface};
use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    },
    time::Duration,
};
use tokio_util::sync::CancellationToken;

use crate::{data::parsed_packet::ParsedPacket, Message};

#[derive(Debug)]
pub struct PacketSubscription {
    pub receiver: Arc<Mutex<Receiver<ParsedPacket>>>,
    pub token: CancellationToken,
}

impl PacketSubscription {
    pub fn new(rx: Arc<Mutex<Receiver<ParsedPacket>>>, token: CancellationToken) -> Self {
        PacketSubscription {
            receiver: rx,
            token,
        }
    }

    pub fn sniff(tx: Sender<ParsedPacket>, interface: NetworkInterface, token: CancellationToken) {
        if let Ok(Ethernet(_, mut rx)) = datalink::channel(&interface, Config::default()) {
            while !token.is_cancelled() {
                if let Ok(packet) = rx.next() {
                    match ParsedPacket::parse(packet) {
                        Some(p) => {
                            let _ = tx.blocking_send(p);
                        }
                        None => {}
                    }
                }
            }
        }
    }
}

impl Recipe for PacketSubscription {
    type Output = Message;

    fn hash(&self, state: &mut iced_futures::core::Hasher) {
        use std::hash::Hash;
        "PacketSubscription".hash(state)
    }

    fn stream(
        self: Box<Self>,
        _input: iced_futures::subscription::EventStream,
    ) -> iced_futures::BoxStream<Self::Output> {
        Box::pin(stream::unfold(
            (self.receiver.clone(), self.token.clone()),
            |(r, t)| async move {
                if !t.is_cancelled() {
                    let mut rx = r.lock().await;
                    let mut buffer = Vec::with_capacity(rx.len());
                    let limit = buffer.capacity();
                    let _ = rx.recv_many(&mut buffer, limit).await;
                    tokio::select! {
                        _ = t.cancelled() => {
                            // drain channel instantly on cancel
                            return Some((Message::PacketsDrained(buffer), (r.clone(), t.clone())));
                        }
                        _ = tokio::time::sleep(Duration::from_millis(100)) => {
                            // no cancel -> buffer longer
                            return Some((Message::PacketsReceived(buffer), (r.clone(), t.clone())));
                        }
                    }
                } else {
                    None
                }
            },
        ))
    }
}
