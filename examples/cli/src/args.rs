use std::net::SocketAddrV4;

use clap::{Parser, Subcommand};
use uhppoted::Transport;

use crate::commands;

#[derive(Parser, Debug)]
#[command(name = "cli")]
#[command(author, version, about = "uhppoted-lib-rs CLI v0.0.0", long_about = None)]
pub struct Args {
    #[arg(long, global = true)]
    pub debug: bool,

    #[arg(long, global = true)]
    pub bind: Option<String>,

    #[arg(long, global = true)]
    pub broadcast: Option<String>,

    #[arg(long, global = true)]
    pub listen: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "find-controllers")]
    FindControllers(FindControllerArgs),

    #[command(name = "get-controller")]
    GetController(GetControllerArgs),
}

#[derive(clap::Args, Debug, Clone, Copy)]
pub struct TransportArgs {
    #[arg(long, group = "transport")]
    pub tcp: bool,

    #[arg(long, group = "transport")]
    pub udp: bool,
}

impl From<TransportArgs> for Option<Transport> {
    fn from(args: TransportArgs) -> Self {
        if args.tcp {
            Some(Transport::Tcp)
        } else if args.udp {
            Some(Transport::Udp)
        } else {
            None
        }
    }
}

#[derive(clap::Args, Debug)]
pub struct FindControllerArgs {}

#[derive(clap::Args, Debug)]
pub struct GetControllerArgs {
    #[arg(long)]
    pub controller: u32,

    #[arg(long)]
    pub dest: Option<SocketAddrV4>,

    #[command(flatten)]
    pub transport: TransportArgs,
}

impl Commands {
    pub fn name(&self) -> &'static str {
        match self {
            Commands::FindControllers { .. } => "find-controllers",
            Commands::GetController { .. } => "get-controller",
        }
    }

    pub fn handle(&self, cfg: uhppoted::Config) {
        match self {
            Commands::FindControllers(cmd) => cmd.handle(cfg),
            Commands::GetController(cmd) => cmd.handle(cfg),
        }
    }

    pub fn list() -> Vec<&'static str> {
        vec!["find-controllers", "get-controller"]
    }
}

impl FindControllerArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::find_controllers(cfg);
    }
}

impl GetControllerArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::get_controller(cfg, self.controller, self.dest, self.transport);
    }
}
