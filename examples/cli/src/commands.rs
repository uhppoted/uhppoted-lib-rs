use std::time::Duration;

use super::args::Args;

pub const COMMANDS: [&Command; 2] = [
    &Command {
        name: "find-controllers",
        func: find_controllers,
    },
    &Command {
        name: "get-controller",
        func: get_controller,
    },
];

pub struct Command {
    pub name: &'static str,
    pub func: fn(args: Args),
}

impl Command {
    pub fn exec(&self, args: Args) {
        println!("{}", self.name);
        (self.func)(args);
    }
}

fn find_controllers(_: Args) {
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

fn get_controller(args: Args) {
    let cfg = uhppoted::Config {
        bind: "192.168.1.125:0".parse().unwrap(),
        broadcast: "192.168.1.255:60000".parse().unwrap(),
        listen: "192.168.1.125:60001".parse().unwrap(),
    };

    match uhppoted::get_controller(&cfg, args.controller, Duration::from_millis(1250)) {
        Ok(response) => println!("{:#?}\n", response),
        Err(e) => error(e),
    };
}

fn error(e: uhppoted::error::Error) {
    println!();
    println!("   *** ERROR: {e}");
    println!();
}
