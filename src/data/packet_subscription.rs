use std::sync::Arc;

use iced::futures::stream;
use iced_futures::subscription::Recipe;
use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    },
    time::{self, Duration},
};
use tokio_util::sync::CancellationToken;

use crate::Message;

use super::postal_packet::PostalPacket;

#[derive(Debug)]
pub struct PacketSubscription {
    pub receiver: Arc<Mutex<Receiver<String>>>,
}

impl PacketSubscription {
    pub fn new(rx: Arc<Mutex<Receiver<String>>>) -> Self {
        PacketSubscription { receiver: rx }
    }

    pub async fn sniff(tx: Arc<Mutex<Sender<String>>>, token: CancellationToken) {
        let sender = tx.lock().await;
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let _ = sender.send("Plink".to_string()).await;
                },
                _= token.cancelled() => {
                    println!("Stopped sniffing");
                    break;
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
        Box::pin(stream::unfold(self.receiver.clone(), |r| async move {
            let mut rx = r.lock().await;
            match rx.recv().await {
                Some(s) => Some((Message::PacketReceived(PostalPacket::new(s)), r.clone())),
                None => None,
            }
        }))
    }
}
