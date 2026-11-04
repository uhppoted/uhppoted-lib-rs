use chrono::NaiveDateTime;
use std::net::Ipv4Addr;
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

#[path = "net/tcp.rs"]
pub(crate) mod tcp;

#[path = "structs.rs"]
pub(crate) mod structs;

#[path = "controller.rs"]
pub(crate) mod controller;

pub(crate) type Msg = [u8; 64];

pub type Config = controller::Config;
pub type Controller = controller::Controller;
pub type Transport = controller::Transport;
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
                MAC_address: response.MAC_address,
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
    let transport = controller.transport();

    let request = encode::get_controller_request(id)?;

    let reply = match dest {
        Some(addr) => match transport {
            Some(controller::Transport::Tcp) => tcp::send_to(cfg, addr, &request, timeout)?,
            _ => udp::send_to(cfg, addr, &request, timeout)?,
        },
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
        MAC_address: response.MAC_address,
        version: response.version,
        date: response.date,
    })
}

#[allow(non_snake_case)]
pub fn set_IPv4<T: controller::ControllerRef>(
    cfg: &Config,
    controller: T,
    address: Ipv4Addr,
    netmask: Ipv4Addr,
    gateway: Ipv4Addr,
    timeout: Duration,
) -> Result<bool> {
    let id = controller.id();
    let dest = controller.dest();
    let transport = controller.transport();

    let request = encode::set_IPv4_request(id, address, netmask, gateway)?;

    let reply = match dest {
        Some(addr) => match transport {
            Some(controller::Transport::Tcp) => tcp::send_to(cfg, addr, &request, timeout)?,
            _ => udp::send_to(cfg, addr, &request, timeout)?,
        },
        None => udp::broadcast_to(cfg, &request, timeout)?,
    };

    let response = decode::set_IPv4_response(&reply)?;

    if response.controller != id {
        return Err(error::ErrorKind::InvalidResponse.into());
    }

    Ok(response.ok)
}

pub fn get_time<T: controller::ControllerRef>(cfg: &Config, controller: T, timeout: Duration) -> Result<NaiveDateTime> {
    let id = controller.id();
    let dest = controller.dest();
    let transport = controller.transport();

    let request = encode::get_time_request(id)?;

    let reply = match dest {
        Some(addr) => match transport {
            Some(controller::Transport::Tcp) => tcp::send_to(cfg, addr, &request, timeout)?,
            _ => udp::send_to(cfg, addr, &request, timeout)?,
        },
        None => udp::broadcast_to(cfg, &request, timeout)?,
    };

    let response = decode::get_time_response(&reply)?;

    if response.controller != id {
        return Err(error::ErrorKind::InvalidResponse.into());
    }

    Ok(response.date_time)
}

pub fn set_time<T: controller::ControllerRef>(
    cfg: &Config,
    controller: T,
    datetime: NaiveDateTime,
    timeout: Duration,
) -> Result<NaiveDateTime> {
    let id = controller.id();
    let dest = controller.dest();
    let transport = controller.transport();

    let request = encode::set_time_request(id, datetime)?;

    let reply = match dest {
        Some(addr) => match transport {
            Some(controller::Transport::Tcp) => tcp::send_to(cfg, addr, &request, timeout)?,
            _ => udp::send_to(cfg, addr, &request, timeout)?,
        },
        None => udp::broadcast_to(cfg, &request, timeout)?,
    };

    let response = decode::set_time_response(&reply)?;

    if response.controller != id {
        return Err(error::ErrorKind::InvalidResponse.into());
    }

    Ok(response.date_time)
}
