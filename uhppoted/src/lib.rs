#[path = "encode.rs"]
pub mod encode;

#[path = "net.rs"]
pub mod net;

#[path = "udp.rs"]
pub mod udp;

#[path = "error.rs"]
pub mod error;

pub type Result<T> = std::result::Result<T, error::Error>;

#[derive(Clone, Debug)]
pub struct Config {
    pub bind: String,      // SocketAddrV4 ??
    pub broadcast: String, // SocketAddrV4 ??
    pub listen: String,    // SocketAddrV4 ??
}

pub fn find_controllers(cfg: &Config) -> u64 {
    if let Ok(request) = encode::get_controller_request(0) {
        let _ = udp::broadcast(cfg, &request);
    }

    0
}
