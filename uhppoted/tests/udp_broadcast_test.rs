use std::assert_matches;
use std::time::Duration;

#[path = "stub.rs"]
mod stub;

#[test]
fn test_find_controllers() {
    let stub = stub::start();

    assert_matches!(
        uhppoted::find_controllers(&stub.cfg, Duration::from_millis(250)),
        Ok(_)
    );
}

#[test]
fn test_get_controller() {
    let stub = stub::start();

    assert_matches!(
        uhppoted::get_controller(&stub.cfg, 405419896, Duration::from_millis(250)),
        Ok(_)
    );
}
