use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;

use uhppoted::Controller;
use uhppoted::ControllerRecord;
use uhppoted::DoorRecord;
use uhppoted::ListenerRecord;
use uhppoted::StatusRecord;
use uhppoted::Transport;

use uhppoted::Alarms;
use uhppoted::Doors;
use uhppoted::Event;

use crate::stub;

const TIMEOUT: Duration = Duration::from_millis(5000);
const DEST: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 60004);

#[test]
fn get_controller() {
    let stub = stub::udp();

    let expected = ControllerRecord {
        controller: 405419896,
        ip_address: "192.168.1.100".parse().unwrap(),
        subnet_mask: "255.255.255.0".parse().unwrap(),
        gateway: "192.168.1.1".parse().unwrap(),
        MAC_address: "00:12:23:34:45:56".to_string(),
        version: "v8.92".to_string(),
        date: "2018-11-05".parse().unwrap(),
    };

    let controller: u32 = 405419896;

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Udp);
    let result = uhppoted::get_controller(&stub.cfg, c, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
#[allow(non_snake_case)]
fn set_IPv4() {
    let stub = stub::udp();

    let expected = true;

    let controller: u32 = 405419896;
    let address: Ipv4Addr = "192.168.1.125".parse().unwrap();
    let netmask: Ipv4Addr = "255.255.255.0".parse().unwrap();
    let gateway: Ipv4Addr = "192.168.1.1".parse().unwrap();

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Udp);
    let result = uhppoted::set_IPv4(&stub.cfg, c, address, netmask, gateway, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn get_time() {
    let stub = stub::udp();

    let expected = NaiveDateTime::parse_from_str("2025-11-01 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();

    let controller: u32 = 405419896;

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Udp);
    let result = uhppoted::get_time(&stub.cfg, c, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn set_time() {
    let stub = stub::udp();

    let expected = NaiveDateTime::parse_from_str("2025-11-01 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap();

    let controller: u32 = 405419896;
    let date_time: NaiveDateTime = "2025-11-04T12:34:56".parse().unwrap();

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Udp);
    let result = uhppoted::set_time(&stub.cfg, c, date_time, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn get_listener() {
    let stub = stub::udp();

    let expected = ListenerRecord {
        address: "192.168.1.100:60001".parse().unwrap(),
        interval: 13,
    };

    let controller: u32 = 405419896;

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Udp);
    let result = uhppoted::get_listener(&stub.cfg, c, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn set_listener() {
    let stub = stub::udp();

    let expected = true;

    let controller: u32 = 405419897;
    let listener: SocketAddrV4 = "192.168.1.100:60001".parse().unwrap();
    let interval: Option<u8> = Some(17);

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Udp);
    let result = uhppoted::set_listener(&stub.cfg, c, listener, interval, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn get_door() {
    let stub = stub::udp();

    let expected = DoorRecord {
        door: 4,
        mode: 3,
        delay: 7,
    };

    let controller: u32 = 405419896;
    let door: u8 = 4;

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Udp);
    let result = uhppoted::get_door(&stub.cfg, c, door, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn set_door() {
    let stub = stub::udp();

    let expected = DoorRecord {
        door: 4,
        mode: 2,
        delay: 17,
    };

    let controller: u32 = 405419896;
    let door: u8 = 4;
    let mode: u8 = 2;
    let delay: u8 = 17;

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Udp);
    let result = uhppoted::set_door(&stub.cfg, c, door, mode, delay, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn set_door_passcodes() {
    let stub = stub::udp();

    let expected = true;

    let controller: u32 = 405419896;
    let door: u8 = 4;
    let passcodes: Vec<u32> = vec![12345, 54321, 999999, 0];

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Udp);
    let result = uhppoted::set_door_passcodes(&stub.cfg, c, door, passcodes, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn open_door() {
    let stub = stub::udp();

    let expected = true;

    let controller: u32 = 405419896;
    let door: u8 = 4;

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Udp);
    let result = uhppoted::open_door(&stub.cfg, c, door, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn get_status() {
    let stub = stub::udp();

    let expected = StatusRecord {
        system_datetime: NaiveDateTime::parse_from_str("2022-08-23 09:49:39", "%Y-%m-%d %H:%M:%S").unwrap(),
        system_error: 3,
        system_special_info: 39,

        doors: Doors {
            open: HashMap::from([(1, false), (2, true), (3, false), (4, false)]),
            button: HashMap::from([(1, false), (2, false), (3, false), (4, true)]),
            unlocked: HashMap::from([(1, true), (2, true), (3, true), (4, false)]),
        },

        alarms: Alarms {
            flags: 9,
            fire: true,
            lock_forced: false,
        },

        event: Some(Event {
            index: 78,
            event_type: 2,
            access_granted: true,
            door: 3,
            direction: 1,
            card: 8165537,
            timestamp: Some(NaiveDateTime::parse_from_str("2022-08-23 09:47:06", "%Y-%m-%d %H:%M:%S").unwrap()),
            reason: 44,
        }),
    };

    let controller: u32 = 405419896;

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Udp);
    let result = uhppoted::get_status(&stub.cfg, c, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn get_status_no_event() {
    let stub = stub::udp();

    let expected = StatusRecord {
        system_datetime: NaiveDateTime::parse_from_str("2025-11-23 14:37:53", "%Y-%m-%d %H:%M:%S").unwrap(),
        system_error: 27,
        system_special_info: 39,

        doors: Doors {
            open: HashMap::from([(1, true), (2, false), (3, true), (4, true)]),
            button: HashMap::from([(1, true), (2, true), (3, false), (4, true)]),
            unlocked: HashMap::from([(1, true), (2, true), (3, true), (4, false)]),
        },

        alarms: Alarms {
            flags: 9,
            fire: true,
            lock_forced: false,
        },

        event: None,
    };

    let controller: u32 = 405419897;

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Udp);
    let result = uhppoted::get_status(&stub.cfg, c, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn get_cards() {
    let stub = stub::udp();

    let expected = 13579;

    let controller: u32 = 405419896;

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Udp);
    let result = uhppoted::get_cards(&stub.cfg, c, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}
