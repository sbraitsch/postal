use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum PostalOptions {
    Autoscroll,
    Tcp,
    Udp,
    Icmp,
    Http,
    Https,
}

impl PostalOptions {
    pub fn as_map() -> HashMap<PostalOptions, bool> {
        let mut map = HashMap::new();
        map.insert(PostalOptions::Autoscroll, false);
        map.insert(PostalOptions::Tcp, true);
        map.insert(PostalOptions::Udp, true);
        map.insert(PostalOptions::Icmp, false);
        map.insert(PostalOptions::Http, false);
        map.insert(PostalOptions::Https, false);
        map
    }
}

impl ToString for PostalOptions {
    fn to_string(&self) -> String {
        match self {
            PostalOptions::Autoscroll => "Autoscroll".to_string(),
            PostalOptions::Tcp => "TCP".to_string(),
            PostalOptions::Udp => "UDP".to_string(),
            PostalOptions::Icmp => "ICMP".to_string(),
            PostalOptions::Http => "HTTP".to_string(),
            PostalOptions::Https => "HTTPS".to_string(),
        }
    }
}
