use chrono::NaiveDateTime;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;

use uhppoted::Controller;
use uhppoted::Transport;

const TIMEOUT: Duration = Duration::from_millis(1250);

pub fn find_controllers(cfg: uhppoted::Config) {
    match uhppoted::find_controllers(&cfg, TIMEOUT) {
        Ok(list) => {
            for response in list {
                println!("{:#?}\n", response)
            }
        }
        Err(e) => error(e),
    };
}

pub fn get_controller(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: impl Into<Option<Transport>>,
) {
    let response = if let Some(addr) = dest {
        let c = if let Some(t) = transport.into() {
            Controller::new(controller).with_address(addr).with_transport(t)
        } else {
            Controller::new(controller).with_address(addr)
        };

        uhppoted::get_controller(&cfg, c, TIMEOUT)
    } else {
        uhppoted::get_controller(&cfg, controller, TIMEOUT)
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

#[allow(non_snake_case)]
pub fn set_IPv4(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: impl Into<Option<Transport>> + std::fmt::Debug,
    address: Ipv4Addr,
    netmask: Ipv4Addr,
    gateway: Ipv4Addr,
) {
    let response = if let Some(addr) = dest {
        let c = if let Some(t) = transport.into() {
            Controller::new(controller).with_address(addr).with_transport(t)
        } else {
            Controller::new(controller).with_address(addr)
        };

        uhppoted::set_IPv4(&cfg, c, address, netmask, gateway, TIMEOUT)
    } else {
        uhppoted::set_IPv4(&cfg, controller, address, netmask, gateway, TIMEOUT)
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn get_time(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: impl Into<Option<Transport>>,
) {
    let response = if let Some(addr) = dest {
        let c = if let Some(t) = transport.into() {
            Controller::new(controller).with_address(addr).with_transport(t)
        } else {
            Controller::new(controller).with_address(addr)
        };

        uhppoted::get_time(&cfg, c, TIMEOUT)
    } else {
        uhppoted::get_time(&cfg, controller, TIMEOUT)
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn set_time(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: impl Into<Option<Transport>>,
    datetime: NaiveDateTime,
) {
    let response = if let Some(addr) = dest {
        let c = if let Some(t) = transport.into() {
            Controller::new(controller).with_address(addr).with_transport(t)
        } else {
            Controller::new(controller).with_address(addr)
        };

        uhppoted::set_time(&cfg, c, datetime, TIMEOUT)
    } else {
        uhppoted::set_time(&cfg, controller, datetime, TIMEOUT)
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

fn error(e: uhppoted::error::Error) {
    println!();
    println!("   *** ERROR: {e}");
    println!();
}
