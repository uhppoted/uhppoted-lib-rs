use chrono::{NaiveDate, NaiveDateTime};
use regex::Regex;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;

use uhppoted::Controller;
use uhppoted::Transport;

const TIMEOUT: Duration = Duration::from_millis(1250);

fn resolve(id: u32, dest: Option<SocketAddrV4>, transport: Option<Transport>) -> Option<Controller> {
    if let Some(addr) = dest {
        if let Some(t) = transport.into() {
            Some(Controller::new(id).with_address(addr).with_transport(t))
        } else {
            Some(Controller::new(id).with_address(addr))
        }
    } else {
        None
    }
}

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
    transport: Option<Transport>,
) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::get_controller(&cfg, c, TIMEOUT),
        None => uhppoted::get_controller(&cfg, controller, TIMEOUT),
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
    transport: Option<Transport>,
    address: Ipv4Addr,
    netmask: Ipv4Addr,
    gateway: Ipv4Addr,
) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::set_IPv4(&cfg, c, address, netmask, gateway, TIMEOUT),
        None => uhppoted::set_IPv4(&cfg, controller, address, netmask, gateway, TIMEOUT),
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn get_time(cfg: uhppoted::Config, controller: u32, dest: Option<SocketAddrV4>, transport: Option<Transport>) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::get_time(&cfg, c, TIMEOUT),
        None => uhppoted::get_time(&cfg, controller, TIMEOUT),
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
    transport: Option<Transport>,
    datetime: NaiveDateTime,
) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::set_time(&cfg, c, datetime, TIMEOUT),
        None => uhppoted::set_time(&cfg, controller, datetime, TIMEOUT),
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn get_listener(cfg: uhppoted::Config, controller: u32, dest: Option<SocketAddrV4>, transport: Option<Transport>) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::get_listener(&cfg, c, TIMEOUT),
        None => uhppoted::get_listener(&cfg, controller, TIMEOUT),
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn set_listener(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: Option<Transport>,
    address: SocketAddrV4,
    interval: Option<u8>,
) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::set_listener(&cfg, c, address, interval, TIMEOUT),
        None => uhppoted::set_listener(&cfg, controller, address, interval, TIMEOUT),
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn get_door(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: Option<Transport>,
    door: u8,
) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::get_door(&cfg, c, door, TIMEOUT),
        None => uhppoted::get_door(&cfg, controller, door, TIMEOUT),
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn set_door(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: Option<Transport>,
    door: u8,
    mode: u8,
    delay: u8,
) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::set_door(&cfg, c, door, mode, delay, TIMEOUT),
        None => uhppoted::set_door(&cfg, controller, door, mode, delay, TIMEOUT),
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn set_door_passcodes(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: Option<Transport>,
    door: u8,
    passcodes: &str,
) {
    let pincodes: Vec<u32> = passcodes
        .split(',')
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect();

    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::set_door_passcodes(&cfg, c, door, pincodes, TIMEOUT),
        None => uhppoted::set_door_passcodes(&cfg, controller, door, pincodes, TIMEOUT),
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn open_door(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: Option<Transport>,
    door: u8,
) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::open_door(&cfg, c, door, TIMEOUT),
        None => uhppoted::open_door(&cfg, controller, door, TIMEOUT),
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn get_status(cfg: uhppoted::Config, controller: u32, dest: Option<SocketAddrV4>, transport: Option<Transport>) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::get_status(&cfg, c, TIMEOUT),
        None => uhppoted::get_status(&cfg, controller, TIMEOUT),
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn get_cards(cfg: uhppoted::Config, controller: u32, dest: Option<SocketAddrV4>, transport: Option<Transport>) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::get_cards(&cfg, c, TIMEOUT),
        None => uhppoted::get_cards(&cfg, controller, TIMEOUT),
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn get_card(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: Option<Transport>,
    card: u32,
) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::get_card(&cfg, c, card, TIMEOUT),
        None => uhppoted::get_card(&cfg, controller, card, TIMEOUT),
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn get_card_at_index(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: Option<Transport>,
    index: u32,
) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::get_card_at_index(&cfg, c, index, TIMEOUT),
        None => uhppoted::get_card_at_index(&cfg, controller, index, TIMEOUT),
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn put_card(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: Option<Transport>,
    card: u32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    doors: &str,
    pin: Option<u32>,
    first_card: Option<String>,
) {
    let mut permissions = HashMap::from([(1, 0), (2, 0), (3, 0), (4, 0)]);
    let mut firstcard = HashMap::from([(1, false), (2, false), (3, false), (4, false)]);

    let re = Regex::new(r"\b([1-4])(?::(\d+))?\b").unwrap();
    for matches in re.captures_iter(doors) {
        let door: u8 = matches[1].parse().unwrap();
        let value: u8 = matches.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(1);

        permissions.insert(door, value);
    }

    if let Some(s) = &first_card {
        let re = Regex::new(r"\b([1-4])\b").unwrap();
        for matches in re.captures_iter(s) {
            let door: u8 = matches[1].parse().unwrap();
            firstcard.insert(door, true);
        }
    }

    let _card = uhppoted::CardRecord {
        card,
        start_date,
        end_date,
        permissions,
        PIN: pin,
        first_card: firstcard,
    };

    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::put_card(&cfg, c, _card, TIMEOUT),
        None => uhppoted::put_card(&cfg, controller, _card, TIMEOUT),
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

pub fn delete_card(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: Option<Transport>,
    card: u32,
) {
    let response = match resolve(controller, dest, transport) {
        Some(c) => uhppoted::delete_card(&cfg, c, card, TIMEOUT),
        None => uhppoted::delete_card(&cfg, controller, card, TIMEOUT),
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
