use std::net::SocketAddrV4;

use super::commands;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "cli")]
#[command(author, version, about = "uhppoted-lib-rs CLI v0.0.0", long_about = None)]
pub struct ArgsX {
    #[arg(long, global = true)]
    pub debug: bool,

    #[arg(long, global = true)]
    pub bind: Option<String>,

    #[arg(long, global = true)]
    pub broadcast: Option<String>,

    #[arg(long, global = true)]
    pub listen: Option<String>,

    #[command(subcommand)]
    pub command: Option<CommandsX>,
}

#[derive(Subcommand, Debug)]
pub enum CommandsX {
    #[command(name = "find-controllers")]
    FindControllers(FindControllerArgs),

    #[command(name = "get-controller")]
    GetController(GetControllerArgs),
}

#[derive(Args, Debug)]
pub struct FindControllerArgs {}

#[derive(Args, Debug)]
pub struct GetControllerArgs {
    #[arg(long)]
    pub controller: u32,

    #[arg(long)]
    pub dest: Option<SocketAddrV4>,
}

impl CommandsX {
    pub fn name(&self) -> &'static str {
        match self {
            CommandsX::FindControllers { .. } => "find-controllers",
            CommandsX::GetController { .. } => "get-controller",
        }
    }

    pub fn handle(&self, cfg: uhppoted::Config) {
        match self {
            CommandsX::FindControllers(cmd) => cmd.handle(cfg),
            CommandsX::GetController(cmd) => cmd.handle(cfg),
        }
    }
}

impl FindControllerArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::find_controllers(cfg);
    }
}

impl GetControllerArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::get_controller(cfg, self.controller, self.dest);
    }
}
