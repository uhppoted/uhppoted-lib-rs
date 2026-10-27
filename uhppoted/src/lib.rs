use std::time::Duration;

pub type Controller = structs::Controller;

pub type Result<T> = std::result::Result<T, error::Error>;

#[path = "error.rs"]
pub mod error;

#[path = "codec/encode.rs"]
pub(crate) mod encode;

#[path = "codec/decode.rs"]
pub(crate) mod decode;

#[path = "net.rs"]
pub(crate) mod net;

#[path = "udp.rs"]
pub(crate) mod udp;

#[path = "structs.rs"]
pub(crate) mod structs;

pub(crate) type Msg = [u8; 64];

#[derive(Clone, Debug)]
pub struct Config {
    pub bind: std::net::SocketAddrV4,
    pub broadcast: std::net::SocketAddrV4,
    pub listen: std::net::SocketAddrV4,
}

pub fn find_controllers(cfg: &Config, timeout: Duration) -> Result<Vec<Controller>> {
    let request = encode::get_controller_request(0)?;

    match udp::broadcast(cfg, &request, timeout) {
        Ok(replies) => Ok(replies
            .iter()
            .map(decode::get_controller_response)
            .filter_map(|reply| reply.ok())
            .map(|response| Controller {
                controller: response.controller,
                ip_address: response.ip_address,
                subnet_mask: response.subnet_mask,
                gateway: response.gateway,
                mac_address: response.mac_address,
                version: response.version,
                date: response.date,
            })
            .collect::<Vec<_>>()),

        Err(e) => Err(e),
    }
}

pub fn get_controller(cfg: &Config, controller: u32, timeout: Duration) -> Result<Controller> {
    let request = encode::get_controller_request(controller)?;
    let reply = udp::broadcast_to(cfg, &request, timeout)?;
    let response = decode::get_controller_response(&reply)?;

    if response.controller != controller {
        return Err(error::ErrorKind::InvalidResponse.into());
    }

    Ok(Controller {
        controller: response.controller,
        ip_address: response.ip_address,
        subnet_mask: response.subnet_mask,
        gateway: response.gateway,
        mac_address: response.mac_address,
        version: response.version,
        date: response.date,
    })
}
