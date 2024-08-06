use core::fmt;

#[derive(Debug, Clone)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Other,
}

impl Protocol {
    pub fn from_u8(val: u8) -> Self {
        match val {
            6 => Self::Tcp,
            17 => Self::Udp,
            1 => Self::Icmp,
            _ => Self::Other,
        }
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Protocol::Tcp => "TCP",
            Protocol::Udp => "UDP",
            Protocol::Icmp => "ICMP",
            Protocol::Other => "Some weird shit",
        };
        write!(f, "{name}")
    }
}
