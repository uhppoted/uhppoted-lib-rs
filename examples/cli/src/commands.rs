use std::net::SocketAddrV4;
use std::time::Duration;

use uhppoted::Controller;
use uhppoted::Transport;

pub fn find_controllers(cfg: uhppoted::Config) {
    match uhppoted::find_controllers(&cfg, Duration::from_millis(1250)) {
        Ok(list) => {
            for response in list {
                println!("{:#?}\n", response)
            }
        }
        Err(e) => error(e),
    };
}

pub fn get_controller(
    cfg: uhppoted::Config,
    controller: u32,
    dest: Option<SocketAddrV4>,
    transport: impl Into<Option<Transport>>,
) {
    let response = if let Some(addr) = dest {
        let c = if let Some(t) = transport.into() {
            Controller::new(controller).with_address(addr).with_transport(t)
        } else {
            Controller::new(controller).with_address(addr)
        };

        uhppoted::get_controller(&cfg, c, Duration::from_millis(1250))
    } else {
        uhppoted::get_controller(&cfg, controller, Duration::from_millis(1250))
    };

    match response {
        Ok(reply) => println!("{:#?}\n", reply),
        Err(e) => error(e),
    };
}

fn error(e: uhppoted::error::Error) {
    println!();
    println!("   *** ERROR: {e}");
    println!();
}
