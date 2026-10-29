use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub bind: SocketAddrV4,
    pub broadcast: SocketAddrV4,
    pub listen: SocketAddrV4,
}

#[derive(Clone, Copy, Debug)]
pub struct Controller {
    pub controller: u32,
    pub address: Option<std::net::SocketAddrV4>,
    pub transport: Option<Transport>,
}

pub trait ControllerRef {
    fn id(&self) -> u32;
    fn dest(&self) -> Option<SocketAddrV4>;
    fn transport(&self) -> Option<Transport>;
}

#[derive(Clone, Copy, Debug)]
pub enum Transport {
    Udp,
    Tcp,
}

impl Config {
    pub fn new() -> Self {
        Self {
            bind: SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0),
            broadcast: SocketAddrV4::new(Ipv4Addr::BROADCAST, 60000),
            listen: SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 60001),
        }
    }

    pub fn with_bind_addr(mut self, address: std::net::SocketAddrV4) -> Self {
        self.bind = address;
        self
    }

    pub fn with_broadcast_addr(mut self, address: std::net::SocketAddrV4) -> Self {
        self.broadcast = address;
        self
    }

    pub fn with_listen_addr(mut self, address: std::net::SocketAddrV4) -> Self {
        self.listen = address;
        self
    }
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

impl ControllerRef for u32 {
    fn id(&self) -> u32 {
        *self
    }

    fn dest(&self) -> Option<SocketAddrV4> {
        None
    }

    fn transport(&self) -> Option<Transport> {
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

    fn transport(&self) -> Option<Transport> {
        self.transport
    }
}
