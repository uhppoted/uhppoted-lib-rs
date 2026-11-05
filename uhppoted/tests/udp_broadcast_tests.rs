use chrono::NaiveDateTime;
use std::net::Ipv4Addr;
use std::time::Duration;

use uhppoted::ControllerRecord;
use uhppoted::ListenerRecord;

#[path = "stub.rs"]
mod stub;

const TIMEOUT: Duration = Duration::from_millis(250);

#[test]
fn test_find_controllers() {
    let stub = stub::start();

    let expected = vec![
        ControllerRecord {
            controller: 201020304,
            ip_address: "192.168.1.101".parse().unwrap(),
            subnet_mask: "255.255.255.0".parse().unwrap(),
            gateway: "192.168.1.1".parse().unwrap(),
            MAC_address: "52:fd:fc:07:21:82".to_string(),
            version: "v6.62".to_string(),
            date: "2020-01-01".parse().unwrap(),
        },
        ControllerRecord {
            controller: 303986753,
            ip_address: "192.168.1.100".parse().unwrap(),
            subnet_mask: "255.255.255.0".parse().unwrap(),
            gateway: "192.168.1.1".parse().unwrap(),
            MAC_address: "52:fd:fc:07:21:82".to_string(),
            version: "v8.92".to_string(),
            date: "2019-08-15".parse().unwrap(),
        },
        ControllerRecord {
            controller: 405419896,
            ip_address: "192.168.1.100".parse().unwrap(),
            subnet_mask: "255.255.255.0".parse().unwrap(),
            gateway: "192.168.1.1".parse().unwrap(),
            MAC_address: "00:12:23:34:45:56".to_string(),
            version: "v8.92".to_string(),
            date: "2018-11-05".parse().unwrap(),
        },
    ];

    let result = uhppoted::find_controllers(&stub.cfg, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_get_controller() {
    let stub = stub::start();

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

    let result = uhppoted::get_controller(&stub.cfg, controller, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
#[allow(non_snake_case)]
fn test_set_IPv4() {
    let stub = stub::start();

    let expected = true;

    let controller: u32 = 405419896;
    let address: Ipv4Addr = "192.168.1.100".parse().unwrap();
    let netmask: Ipv4Addr = "255.255.255.0".parse().unwrap();
    let gateway: Ipv4Addr = "192.168.1.1".parse().unwrap();

    let result = uhppoted::set_IPv4(&stub.cfg, controller, address, netmask, gateway, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
#[allow(non_snake_case)]
fn test_get_time() {
    let stub = stub::start();

    let expected: NaiveDateTime = "2025-11-01T12:34:56".parse().unwrap();

    let controller: u32 = 405419896;

    let result = uhppoted::get_time(&stub.cfg, controller, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
#[allow(non_snake_case)]
fn test_set_time() {
    let stub = stub::start();

    let expected: NaiveDateTime = "2025-11-01T12:34:56".parse().unwrap();

    let controller: u32 = 405419896;
    let datetime: NaiveDateTime = "2025-11-04T12:34:56".parse().unwrap();

    let result = uhppoted::set_time(&stub.cfg, controller, datetime, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_get_listener() {
    let stub = stub::start();

    let expected = ListenerRecord {
        address: "192.168.1.100:60001".parse().unwrap(),
        interval: 13,
    };

    let controller: u32 = 405419896;

    let result = uhppoted::get_listener(&stub.cfg, controller, TIMEOUT).unwrap();

    assert_eq!(result, expected);
}
