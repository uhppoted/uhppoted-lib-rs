use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;

#[path = "error.rs"]
pub mod error;

#[path = "codec/codec.rs"]
pub(crate) mod codec;

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
pub type ListenerRecord = structs::ListenerRecord;
pub type StatusRecord = structs::StatusRecord;
pub type DoorRecord = structs::DoorRecord;

pub type Doors = structs::Doors;
pub type Alarms = structs::Alarms;
pub type Event = structs::Event;

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

pub fn get_listener<T: controller::ControllerRef>(
    cfg: &Config,
    controller: T,
    timeout: Duration,
) -> Result<ListenerRecord> {
    let id = controller.id();
    let dest = controller.dest();
    let transport = controller.transport();

    let request = encode::get_listener_request(id)?;

    let reply = match dest {
        Some(addr) => match transport {
            Some(controller::Transport::Tcp) => tcp::send_to(cfg, addr, &request, timeout)?,
            _ => udp::send_to(cfg, addr, &request, timeout)?,
        },
        None => udp::broadcast_to(cfg, &request, timeout)?,
    };

    let response = decode::get_listener_response(&reply)?;

    if response.controller != id {
        return Err(error::ErrorKind::InvalidResponse.into());
    }

    Ok(ListenerRecord {
        address: SocketAddrV4::new(response.address, response.port),
        interval: response.interval,
    })
}

pub fn set_listener<T: controller::ControllerRef>(
    cfg: &Config,
    controller: T,
    address: SocketAddrV4,
    interval: Option<u8>,
    timeout: Duration,
) -> Result<bool> {
    let id = controller.id();
    let dest = controller.dest();
    let transport = controller.transport();

    let request = match interval {
        Some(v) => encode::set_listener_request(id, *address.ip(), address.port(), v)?,
        _ => encode::set_listener_request(id, *address.ip(), address.port(), 0)?,
    };

    let reply = match dest {
        Some(addr) => match transport {
            Some(controller::Transport::Tcp) => tcp::send_to(cfg, addr, &request, timeout)?,
            _ => udp::send_to(cfg, addr, &request, timeout)?,
        },
        None => udp::broadcast_to(cfg, &request, timeout)?,
    };

    let response = decode::set_listener_response(&reply)?;

    if response.controller != id {
        return Err(error::ErrorKind::InvalidResponse.into());
    }

    Ok(response.ok)
}

pub fn get_door<T: controller::ControllerRef>(
    cfg: &Config,
    controller: T,
    door: u8,
    timeout: Duration,
) -> Result<DoorRecord> {
    let id = controller.id();
    let dest = controller.dest();
    let transport = controller.transport();

    let request = encode::get_door_request(id, door)?;

    let reply = match dest {
        Some(addr) => match transport {
            Some(controller::Transport::Tcp) => tcp::send_to(cfg, addr, &request, timeout)?,
            _ => udp::send_to(cfg, addr, &request, timeout)?,
        },
        None => udp::broadcast_to(cfg, &request, timeout)?,
    };

    let response = decode::get_door_response(&reply)?;

    if response.controller != id || response.door != door {
        return Err(error::ErrorKind::InvalidResponse.into());
    }

    Ok(DoorRecord {
        door: response.door,
        mode: response.mode,
        delay: response.delay,
    })
}

pub fn get_status<T: controller::ControllerRef>(
    cfg: &Config,
    controller: T,
    timeout: Duration,
) -> Result<StatusRecord> {
    let id = controller.id();
    let dest = controller.dest();
    let transport = controller.transport();

    let request = encode::get_status_request(id)?;

    let reply = match dest {
        Some(addr) => match transport {
            Some(controller::Transport::Tcp) => tcp::send_to(cfg, addr, &request, timeout)?,
            _ => udp::send_to(cfg, addr, &request, timeout)?,
        },
        None => udp::broadcast_to(cfg, &request, timeout)?,
    };

    let response = decode::get_status_response(&reply)?;

    if response.controller != id {
        return Err(error::ErrorKind::InvalidResponse.into());
    }

    Ok(StatusRecord {
        system_datetime: NaiveDateTime::new(response.system_date, response.system_time),
        system_error: response.system_error,
        system_special_info: response.special_info,

        doors: Doors {
            open: HashMap::from([
                (1, response.door_1_open),
                (2, response.door_2_open),
                (3, response.door_3_open),
                (4, response.door_4_open),
            ]),

            button: HashMap::from([
                (1, response.door_1_button),
                (2, response.door_2_button),
                (3, response.door_3_button),
                (4, response.door_4_button),
            ]),

            unlocked: HashMap::from([
                (1, (response.relays & 0x01) == 0x01),
                (2, (response.relays & 0x02) == 0x02),
                (3, (response.relays & 0x04) == 0x04),
                (4, (response.relays & 0x08) == 0x08),
            ]),
        },

        alarms: Alarms {
            flags: response.inputs,
            fire: (response.inputs & 0x01) == 0x01,
            lock_forced: (response.inputs & 0x02) == 0x02,
        },

        event: Some(Event {
            index: response.event_index,
            event_type: response.event_type,
            access_granted: response.event_access_granted,
            door: response.event_door,
            direction: response.event_direction,
            card: response.event_card,
            timestamp: response.event_timestamp,
            reason: response.event_reason,
        }),
    })
}
