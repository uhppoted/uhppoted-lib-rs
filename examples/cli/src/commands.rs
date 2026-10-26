use std::time::Duration;

pub const COMMANDS: [&Command; 1] = [&Command {
    name: "find-controllers",
    func: find_controllers,
}];

pub struct Command {
    pub name: &'static str,
    pub func: fn(),
}

impl Command {
    pub fn exec(&self) {
        println!("{}", self.name);
        (self.func)();
    }
}

fn find_controllers() {
    let cfg = uhppoted::Config {
        bind: "192.168.1.125:0".parse().unwrap(),
        broadcast: "192.168.1.255:60000".parse().unwrap(),
        listen: "192.168.1.125:60001".parse().unwrap(),
    };

    match uhppoted::find_controllers(&cfg, Duration::from_millis(1250)) {
        Ok(list) => {
            for response in list {
                println!("{:#?}\n", response)
            }
        }
        Err(e) => error(e),
    };
}

fn error(e: uhppoted::error::Error) {
    println!();
    println!("   *** ERROR: {e}");
    println!();
}
