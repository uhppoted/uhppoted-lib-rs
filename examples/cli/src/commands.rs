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
        bind: "192.168.1.125:0".to_string(),
        broadcast: "192.168.1.255:60000".to_string(),
        listen: "192.168.1.125:60001".to_string(),
    };

    uhppoted::find_controllers(&cfg);
}
