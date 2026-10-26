use std::time::Duration;

#[test]
fn test_find_controllers() {
    let cfg = uhppoted::Config {
        bind: "192.168.1.125:0".to_string(),
        broadcast: "192.168.1.255:60000".to_string(),
        listen: "192.168.1.125:60001".to_string(),
    };

    assert_eq!(
        uhppoted::find_controllers(&cfg, Duration::from_millis(250)),
        0
    );
}
