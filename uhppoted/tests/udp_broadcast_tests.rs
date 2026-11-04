use std::time::Duration;

use uhppoted::ControllerRecord;

#[path = "stub.rs"]
mod stub;

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

    let result = uhppoted::find_controllers(&stub.cfg, Duration::from_millis(250)).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_get_controller() {
    // let stub = stub::start();
    //
    // assert_matches!(
    //     uhppoted::get_controller(&stub.cfg, 405419896, Duration::from_millis(250)),
    //     Ok(_)
    // );
}
