use std::assert_matches;
use std::time::Duration;

#[test]
fn test_find_controllers() {
    let cfg = uhppoted::Config {
        bind: "192.168.1.125:0".parse().unwrap(),
        broadcast: "192.168.1.255:60000".parse().unwrap(),
        listen: "192.168.1.125:60001".parse().unwrap(),
    };

    assert_matches!(
        uhppoted::find_controllers(&cfg, Duration::from_millis(250)),
        Ok(_)
    );
}
