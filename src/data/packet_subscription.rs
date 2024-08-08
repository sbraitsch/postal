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
}

impl PacketSubscription {
    pub fn new(rx: Arc<Mutex<Receiver<ParsedPacket>>>) -> Self {
        PacketSubscription { receiver: rx }
    }

    pub fn sniff(
        tx: Arc<Mutex<Sender<ParsedPacket>>>,
        interface: NetworkInterface,
        token: CancellationToken,
    ) {
        let sender = tx.blocking_lock();

        if let Ok(Ethernet(_, mut rx)) = datalink::channel(&interface, Config::default()) {
            while !token.is_cancelled() {
                if let Ok(packet) = rx.next() {
                    match ParsedPacket::parse(packet) {
                        Some(p) => {
                            let _ = sender.blocking_send(p);
                        }
                        None => {}
                    }
                }
            }
            println!("Stopped sniffing task.")
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
        Box::pin(stream::unfold(self.receiver.clone(), |r| async move {
            tokio::time::sleep(Duration::from_millis(500)).await;
            let mut rx = r.lock().await;
            let mut buffer = Vec::with_capacity(rx.len());
            let limit = buffer.capacity();
            let _ = rx.recv_many(&mut buffer, limit).await;
            return Some((Message::PacketsReceived(buffer), r.clone()));
        }))
    }
}
