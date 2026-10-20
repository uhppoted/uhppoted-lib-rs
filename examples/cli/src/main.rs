use std::env;
use std::process::ExitCode;

use commands::COMMANDS;

#[path = "commands.rs"]
mod commands;

fn main() -> ExitCode {
    println!("uhppoted-lib-rs CLI v0.0.0");

    let mut args = env::args().skip(1).peekable();

    while let Some(arg) = args.peek() {
        match arg.as_str() {
            "--debug" => {
                args.next();
                println!(">>> uhppoted-lib-rs::debug");
            }

            "--bind" => {
                args.next();
                if let Some(val) = args.next() {
                    println!(">>> uhppoted-lib-rs::bind {}", val);
                }
            }

            "--broadcast" => {
                args.next();
                if let Some(val) = args.next() {
                    println!(">>> uhppoted-lib-rs::broadcast {}", val);
                }
            }

            "--listen" => {
                args.next();
                if let Some(val) = args.next() {
                    println!(">>> uhppoted-lib-rs::listen {}", val);
                }
            }

            _ => break,
        }
    }

    if let Some(cmd) = args.next() {
        let ok = match COMMANDS.iter().find(|c| c.name == cmd) {
            Some(c) => {
                c.exec();
                println!();
                true
            }

            None => {
                println!();
                println!("   *** ERROR: invalid command {cmd}");
                println!();
                false
            }
        };

        if ok {
            return ExitCode::SUCCESS;
        } else {
            return ExitCode::FAILURE;
        }
    }

    // ... no command!
    usage();

    ExitCode::FAILURE
}

fn usage() {
    println!();
    println!("  Usage: cli [--debug] [--bind <address>] [--broadcast <address>] [commands]");
    println!();
    println!("    Options:");
    println!("    --debug                prints sent and received UDP packets to stdout");
    println!("    --bind <address>       IPv4 address to which to bind. Defaults to 0.0.0.0");
    println!("    --broadcast <address>  IPv4 address for UDP broadcast. Defaults to 255.255.255.255:60000");
    println!("    --listen <address>     IPv4 address:port on which to listen for events. Defaults to 0.0.0.0:60001");
    println!();
    println!("    Commands:");
    for c in COMMANDS {
        println!("      {}", c.name);
    }

    println!();
}
