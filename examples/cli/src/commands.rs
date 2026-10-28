use std::time::Duration;

use uhppoted::Controller;

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
    pub func: fn(cfg: uhppoted::Config, args: Args),
}

impl Command {
    pub fn exec(&self, cfg: uhppoted::Config, args: Args) {
        println!("{}", self.name);
        (self.func)(cfg, args);
    }
}

fn find_controllers(cfg: uhppoted::Config, _: Args) {
    match uhppoted::find_controllers(&cfg, Duration::from_millis(1250)) {
        Ok(list) => {
            for response in list {
                println!("{:#?}\n", response)
            }
        }
        Err(e) => error(e),
    };
}

fn get_controller(cfg: uhppoted::Config, args: Args) {
    let controller = Controller::new(args.controller).with_address(args.dest);

    match uhppoted::get_controller(&cfg, controller, Duration::from_millis(1250)) {
        Ok(response) => println!("{:#?}\n", response),
        Err(e) => error(e),
    };

    // match uhppoted::get_controller(&cfg, args.controller, Duration::from_millis(1250)) {
    //     Ok(response) => println!("{:#?}\n", response),
    //     Err(e) => error(e),
    // };
}

fn error(e: uhppoted::error::Error) {
    println!();
    println!("   *** ERROR: {e}");
    println!();
}
