use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct TestStub {
    pub cfg: uhppoted::Config,
}

pub fn start() -> &'static TestStub {
    static STUB: OnceLock<TestStub> = OnceLock::new();

    STUB.get_or_init(|| TestStub {
        cfg: uhppoted::Config {
            bind: "192.168.1.125:0".parse().unwrap(),
            broadcast: "192.168.1.255:60000".parse().unwrap(),
            listen: "192.168.1.125:60001".parse().unwrap(),
        },
    })
}
