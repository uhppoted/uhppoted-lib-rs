use std::net::SocketAddrV4;

#[derive(Clone, Debug)]
pub struct Args {
    pub controller: u32,
    pub dest: SocketAddrV4,
}
