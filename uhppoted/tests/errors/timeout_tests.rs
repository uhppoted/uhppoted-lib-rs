use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;

use crate::stub;

use uhppoted::Controller;
use uhppoted::Transport;

const TIMEOUT: Duration = Duration::from_millis(2500);
const UDP: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 60004);
const TCP: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 60003);

#[test]
fn broadcast_get_controller() {
    let stub = stub::broadcast();
    let controller: u32 = 303986753;
    let result = uhppoted::get_controller(&stub.cfg, controller, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Timeout waiting for response from controller"));
}

#[test]
fn udp_get_controller() {
    let stub = stub::udp();
    let controller: u32 = 303986753;
    let c = Controller::new(controller)
        .with_address(UDP)
        .with_transport(Transport::Udp);

    let result = uhppoted::get_controller(&stub.cfg, c, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Timeout waiting for response from controller"));
}

#[test]
fn tcp_get_controller() {
    let stub = stub::tcp();
    let controller: u32 = 303986753;
    let c = Controller::new(controller)
        .with_address(TCP)
        .with_transport(Transport::Tcp);

    let result = uhppoted::get_controller(&stub.cfg, c, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Timeout waiting for response from controller"));
}
