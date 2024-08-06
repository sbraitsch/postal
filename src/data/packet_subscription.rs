use iced::{futures::SinkExt, subscription, Subscription};
use pcap::{Capture, Device};

use crate::PostalPacket;

pub fn pcap_subscribe<'a>() -> Subscription<PostalPacket> {
    subscription::channel("Packet Stream", 100, |mut output| async move {
        let device = Device::lookup().unwrap().unwrap();
        let capture = Capture::from_device(device).unwrap().snaplen(65535);
        let mut cap = capture.open().unwrap();
        loop {
            let p = PostalPacket::from_packet(cap.next_packet().unwrap());
            if p.port == 443 {
                println!("{p}");
            }
            // todo: batch
            let _ = output.send(p).await;
        }
    })
}
