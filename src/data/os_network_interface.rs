use std::fmt;
use pnet::datalink::NetworkInterface;

#[derive(Debug, Clone)]
pub struct OSNetworkInterface {
    pub interface: NetworkInterface,
}

impl OSNetworkInterface {
    pub fn new(interface: NetworkInterface) -> Self {
        Self {
            interface
        }
    }

    #[cfg(target_os = "windows")]
    pub fn get_identifier(&self) -> &str {
        &self.interface.description
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn get_identifier(&self) -> &str {
        &self.interface.name
    }
}

#[cfg(target_os = "windows")]
impl fmt::Display for OSNetworkInterface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.interface.description)
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.interface.name)
    }
}