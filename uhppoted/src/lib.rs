use std::time::Duration;

#[path = "error.rs"]
pub mod error;

#[path = "codec/encode.rs"]
pub(crate) mod encode;

#[path = "codec/decode.rs"]
pub(crate) mod decode;

#[path = "net/net.rs"]
pub(crate) mod net;

#[path = "net/udp.rs"]
pub(crate) mod udp;

#[path = "structs.rs"]
pub(crate) mod structs;

#[path = "controller.rs"]
pub(crate) mod controller;

pub(crate) type Msg = [u8; 64];

pub type Config = controller::Config;
pub type Controller = controller::Controller;
pub type Result<T> = std::result::Result<T, error::Error>;

pub type ControllerRecord = structs::ControllerRecord;

pub fn find_controllers(cfg: &Config, timeout: Duration) -> Result<Vec<ControllerRecord>> {
    let request = encode::get_controller_request(0)?;

    match udp::broadcast(cfg, &request, timeout) {
        Ok(replies) => Ok(replies
            .iter()
            .map(decode::get_controller_response)
            .filter_map(|reply| reply.ok())
            .map(|response| ControllerRecord {
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

pub fn get_controller<T: controller::ControllerRef>(
    cfg: &Config,
    controller: T,
    timeout: Duration,
) -> Result<ControllerRecord> {
    let id = controller.id();
    let dest = controller.dest();
    let request = encode::get_controller_request(id)?;

    let reply = match dest {
        Some(addr) => udp::send_to(cfg, addr, &request, timeout)?,
        None => udp::broadcast_to(cfg, &request, timeout)?,
    };

    let response = decode::get_controller_response(&reply)?;

    if response.controller != id {
        return Err(error::ErrorKind::InvalidResponse.into());
    }

    Ok(ControllerRecord {
        controller: response.controller,
        ip_address: response.ip_address,
        subnet_mask: response.subnet_mask,
        gateway: response.gateway,
        mac_address: response.mac_address,
        version: response.version,
        date: response.date,
    })
}
