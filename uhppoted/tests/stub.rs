use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct TestStub {
    pub cfg: uhppoted::Config,
}

pub fn start() -> &'static TestStub {
    static STUB: OnceLock<TestStub> = OnceLock::new();

    STUB.get_or_init(|| TestStub {
        cfg: uhppoted::Config {
            bind: "0.0.0.0:0".parse().unwrap(),
            broadcast: "255.255.255.255:60000".parse().unwrap(),
            listen: "0.0.0.0:60001".parse().unwrap(),
        },
    })
}
