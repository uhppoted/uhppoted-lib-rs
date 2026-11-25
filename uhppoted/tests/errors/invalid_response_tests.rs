use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;

use crate::stub;

use uhppoted::Controller;
use uhppoted::Transport;

const TIMEOUT: Duration = Duration::from_millis(500);
const UDP: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 60004);
const TCP: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 60003);

#[test]
#[allow(non_snake_case)]
fn broadcast_incorrect_controller() {
    let stub = stub::broadcast();
    let controller: u32 = 605040302;
    let result = uhppoted::get_controller(&stub.cfg, controller, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Invalid response from controller"));
}

#[test]
#[allow(non_snake_case)]
fn broadcast_invalid_SOM() {
    let stub = stub::broadcast();
    let controller: u32 = 201020304;
    let result = uhppoted::get_controller(&stub.cfg, controller, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Invalid reply start of message byte (18)"));
}

#[test]
#[allow(non_snake_case)]
fn broadcast_invalid_opcode() {
    let stub = stub::broadcast();
    let controller: u32 = 201020304;
    let result = uhppoted::get_time(&stub.cfg, controller, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Invalid reply function code (94)"));
}

#[test]
#[allow(non_snake_case)]
fn udp_invalid_SOM() {
    let stub = stub::udp();
    let controller = Controller::new(201020304)
        .with_address(UDP)
        .with_transport(Transport::Udp);

    let result = uhppoted::get_controller(&stub.cfg, controller, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Invalid reply start of message byte (18)"));
}

#[test]
#[allow(non_snake_case)]
fn udp_invalid_opcode() {
    let stub = stub::udp();
    let controller = Controller::new(201020304)
        .with_address(UDP)
        .with_transport(Transport::Udp);

    let result = uhppoted::get_time(&stub.cfg, controller, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Invalid reply function code (94)"));
}

#[test]
#[allow(non_snake_case)]
fn tcp_invalid_SOM() {
    let stub = stub::tcp();
    let controller = Controller::new(201020304)
        .with_address(TCP)
        .with_transport(Transport::Tcp);

    let result = uhppoted::get_controller(&stub.cfg, controller, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Invalid reply start of message byte (18)"));
}

#[test]
#[allow(non_snake_case)]
fn tcp_invalid_opcode() {
    let stub = stub::tcp();
    let controller = Controller::new(201020304)
        .with_address(TCP)
        .with_transport(Transport::Tcp);

    let result = uhppoted::get_time(&stub.cfg, controller, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Invalid reply function code (94)"));
}
