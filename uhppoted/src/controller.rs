use std::net::SocketAddrV4;

#[derive(Clone, Debug)]
pub struct Config {
    pub bind: std::net::SocketAddrV4,
    pub broadcast: std::net::SocketAddrV4,
    pub listen: std::net::SocketAddrV4,
}

#[derive(Clone, Debug)]
pub struct Controller {
    pub controller: u32,
    pub address: Option<std::net::SocketAddrV4>,
    pub transport: Option<Transport>,
}

impl Controller {
    pub fn new(controller: u32) -> Self {
        Self {
            controller,
            address: None,
            transport: None,
        }
    }

    pub fn with_address(mut self, address: std::net::SocketAddrV4) -> Self {
        self.address = Some(address);
        self
    }

    pub fn with_transport(mut self, transport: Transport) -> Self {
        self.transport = Some(transport);
        self
    }
}

pub trait ControllerRef {
    fn id(&self) -> u32;
    fn dest(&self) -> Option<SocketAddrV4>;
}

#[derive(Clone, Copy, Debug)]
pub enum Transport {
    UDP,
    TCP,
}

impl ControllerRef for u32 {
    fn id(&self) -> u32 {
        *self
    }

    fn dest(&self) -> Option<SocketAddrV4> {
        None
    }
}

impl ControllerRef for Controller {
    fn id(&self) -> u32 {
        self.controller
    }

    fn dest(&self) -> Option<SocketAddrV4> {
        self.address
    }
}
