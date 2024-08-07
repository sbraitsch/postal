use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Filter {
    Tcp,
    Udp,
    Icmp,
    Http,
    Https,
}

impl Filter {
    pub fn as_map() -> HashMap<Filter, bool> {
        let mut map = HashMap::new();
        map.insert(Filter::Tcp, false);
        map.insert(Filter::Udp, false);
        map.insert(Filter::Icmp, false);
        map.insert(Filter::Http, false);
        map.insert(Filter::Https, false);
        map
    }
}

impl ToString for Filter {
    fn to_string(&self) -> String {
        match self {
            Filter::Tcp => "TCP".to_string(),
            Filter::Udp => "UDP".to_string(),
            Filter::Icmp => "ICMP".to_string(),
            Filter::Http => "HTTP".to_string(),
            Filter::Https => "HTTPS".to_string(),
        }
    }
}
