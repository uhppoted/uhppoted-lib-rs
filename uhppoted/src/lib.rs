use std::time::Duration;
#[path = "encode.rs"]
pub mod encode;

#[path = "decode.rs"]
pub mod decode;

#[path = "net.rs"]
pub mod net;

#[path = "udp.rs"]
pub mod udp;

#[path = "error.rs"]
pub mod error;

pub type Result<T> = std::result::Result<T, error::Error>;

pub(crate) type Msg = [u8; 64];

pub type GetControllerResponse = decode::GetControllerResponse;

#[derive(Clone, Debug)]
pub struct Config {
    pub bind: String,      // SocketAddrV4 ??
    pub broadcast: String, // SocketAddrV4 ??
    pub listen: String,    // SocketAddrV4 ??
}

pub fn find_controllers(cfg: &Config, timeout: Duration) -> Result<Vec<GetControllerResponse>> {
    let request = encode::get_controller_request(0)?;

    match udp::broadcast(cfg, &request, timeout) {
        Ok(replies) => {
            return Ok(replies
                .iter()
                .map(|reply| decode::get_controller_response(&reply))
                .filter_map(|reply| reply.ok())
                .collect::<Vec<_>>());
        }

        Err(e) => return Err(e),
    }
}
