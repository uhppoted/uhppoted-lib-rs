use log::{info, warn};
use std::process::ExitCode;

use clap::Parser;

use args::{Args, Commands};

#[path = "commands.rs"]
mod commands;

#[path = "args.rs"]
pub mod args;

fn main() -> ExitCode {
    env_logger::init();

    info!("uhppoted-lib-rs CLI v0.0.0");

    let args = Args::parse();

    // ... config
    let mut cfg = uhppoted::Config::new();

    if let Some(addr) = args.bind {
        cfg = cfg.with_bind_addr(addr.parse().unwrap());
    }

    if let Some(addr) = args.broadcast {
        cfg = cfg.with_broadcast_addr(addr.parse().unwrap());
    }

    if let Some(addr) = args.listen {
        cfg = cfg.with_listen_addr(addr.parse().unwrap());
    }

    // ... execute
    if let Some(cmd) = args.command {
        cmd.handle(cfg);

        return ExitCode::SUCCESS;
    } else {
        warn!("   *** ERROR: invalid command");
    }

    // ... no command!
    usage();

    ExitCode::FAILURE
}

fn usage() {
    println!();
    println!("  Usage: cli [--debug] [--bind <address>] [--broadcast <address>] [command]");
    println!();
    println!("    Options:");
    println!("    --debug                prints sent and received packets to log::DEBUG");
    println!("    --bind <address>       IPv4 address to which to bind. Defaults to 0.0.0.0");
    println!("    --broadcast <address>  IPv4 address for UDP broadcast. Defaults to 255.255.255.255:60000");
    println!("    --listen <address>     IPv4 address:port on which to listen for events. Defaults to 0.0.0.0:60001");
    println!();
    println!("    Commands:");

    for c in Commands::list() {
        println!("      {}", c);
    }

    println!();
}
