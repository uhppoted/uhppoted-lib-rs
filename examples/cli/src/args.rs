use chrono::{Local, NaiveDate};
use std::net::{Ipv4Addr, SocketAddrV4};

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

    #[command(name = "set-IPv4")]
    SetIPv4(SetIPv4Args),

    #[command(name = "get-time")]
    GetTime(GetTimeArgs),

    #[command(name = "set-time")]
    SetTime(SetTimeArgs),

    #[command(name = "get-listener")]
    GetListener(GetListenerArgs),

    #[command(name = "set-listener")]
    SetListener(SetListenerArgs),

    #[command(name = "get-door")]
    GetDoor(GetDoorArgs),

    #[command(name = "set-door")]
    SetDoor(SetDoorArgs),

    #[command(name = "set-door-passcodes")]
    SetDoorPasscodes(SetDoorPasscodesArgs),

    #[command(name = "open-door")]
    OpenDoor(OpenDoorArgs),

    #[command(name = "get-status")]
    GetStatus(GetStatusArgs),

    #[command(name = "get-cards")]
    GetCards(GetCardsArgs),

    #[command(name = "get-card")]
    GetCard(GetCardArgs),

    #[command(name = "get-card-at-index")]
    GetCardAtIndex(GetCardAtIndexArgs),

    #[command(name = "put-card")]
    PutCard(PutCardArgs),

    #[command(name = "delete-card")]
    DeleteCard(DeleteCardArgs),
}

#[derive(clap::Args, Debug)]
pub struct ControllerArgs {
    #[arg(long)]
    pub controller: u32,

    #[arg(long)]
    pub dest: Option<SocketAddrV4>,

    #[arg(long, group = "transport")]
    pub tcp: bool,

    #[arg(long, group = "transport")]
    pub udp: bool,
}

impl ControllerArgs {
    pub fn transport(&self) -> Option<Transport> {
        if self.tcp {
            Some(Transport::Tcp)
        } else if self.udp {
            Some(Transport::Udp)
        } else {
            None
        }
    }
}

// #[derive(clap::Args, Debug, Clone, Copy)]
// pub struct TransportArgs {
//     #[arg(long, group = "transport")]
//     pub tcp: bool,
//
//     #[arg(long, group = "transport")]
//     pub udp: bool,
// }
//
// impl From<TransportArgs> for Option<Transport> {
//     fn from(args: TransportArgs) -> Self {
//         if args.tcp {
//             Some(Transport::Tcp)
//         } else if args.udp {
//             Some(Transport::Udp)
//         } else {
//             None
//         }
//     }
// }

#[derive(clap::Args, Debug)]
pub struct FindControllerArgs {}

#[derive(clap::Args, Debug)]
pub struct GetControllerArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,
}

#[derive(clap::Args, Debug)]
pub struct SetIPv4Args {
    #[command(flatten)]
    pub controller: ControllerArgs,

    #[arg(long)]
    pub address: Ipv4Addr,

    #[arg(long)]
    pub netmask: Ipv4Addr,

    #[arg(long)]
    pub gateway: Ipv4Addr,
}

#[derive(clap::Args, Debug)]
pub struct GetTimeArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,
}

#[derive(clap::Args, Debug)]
pub struct SetTimeArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,
}

#[derive(clap::Args, Debug)]
pub struct GetListenerArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,
}

#[derive(clap::Args, Debug)]
pub struct SetListenerArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,

    #[arg(long)]
    pub address: SocketAddrV4,

    #[arg(long)]
    pub interval: Option<u8>,
}

#[derive(clap::Args, Debug)]
pub struct GetDoorArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,

    #[arg(long)]
    pub door: u8,
}

#[derive(clap::Args, Debug)]
pub struct SetDoorArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,

    #[arg(long)]
    pub door: u8,

    #[arg(long)]
    pub mode: u8,

    #[arg(long)]
    pub delay: u8,
}

#[derive(clap::Args, Debug)]
pub struct SetDoorPasscodesArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,

    #[arg(long)]
    pub door: u8,

    #[arg(long)]
    pub passcodes: String,
}

#[derive(clap::Args, Debug)]
pub struct OpenDoorArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,

    #[arg(long)]
    pub door: u8,
}

#[derive(clap::Args, Debug)]
pub struct GetStatusArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,
}

#[derive(clap::Args, Debug)]
pub struct GetCardsArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,
}

#[derive(clap::Args, Debug)]
pub struct GetCardArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,

    #[arg(long)]
    pub card: u32,
}

#[derive(clap::Args, Debug)]
pub struct GetCardAtIndexArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,

    #[arg(long)]
    pub index: u32,
}

#[derive(clap::Args, Debug)]
pub struct PutCardArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,

    #[arg(long)]
    pub card: u32,

    #[arg(long)]
    pub start_date: NaiveDate,

    #[arg(long)]
    pub end_date: NaiveDate,

    #[arg(long)]
    pub doors: String,

    #[arg(long)]
    pub pin: Option<u32>,

    #[arg(long)]
    pub firstcard: Option<String>,
}

#[derive(clap::Args, Debug)]
pub struct DeleteCardArgs {
    #[command(flatten)]
    pub controller: ControllerArgs,

    #[arg(long)]
    pub card: u32,
}

impl Commands {
    pub fn name(&self) -> &'static str {
        match self {
            Commands::FindControllers { .. } => "find-controllers",
            Commands::GetController { .. } => "get-controller",
            Commands::SetIPv4 { .. } => "set-IPv4",
            Commands::GetTime { .. } => "get-time",
            Commands::SetTime { .. } => "set-time",
            Commands::GetListener { .. } => "get-listener",
            Commands::SetListener { .. } => "set-listener",
            Commands::GetDoor { .. } => "get-door",
            Commands::SetDoor { .. } => "set-door",
            Commands::SetDoorPasscodes { .. } => "set-door-passcodes",
            Commands::OpenDoor { .. } => "open-door",
            Commands::GetStatus { .. } => "get-status",
            Commands::GetCards { .. } => "get-cards",
            Commands::GetCard { .. } => "get-card",
            Commands::GetCardAtIndex { .. } => "get-card-at-index",
            Commands::PutCard { .. } => "put-card",
            Commands::DeleteCard { .. } => "delete-card",
        }
    }

    pub fn handle(&self, cfg: uhppoted::Config) {
        match self {
            Commands::FindControllers(cmd) => cmd.handle(cfg),
            Commands::GetController(cmd) => cmd.handle(cfg),
            Commands::SetIPv4(cmd) => cmd.handle(cfg),
            Commands::GetTime(cmd) => cmd.handle(cfg),
            Commands::SetTime(cmd) => cmd.handle(cfg),
            Commands::GetListener(cmd) => cmd.handle(cfg),
            Commands::SetListener(cmd) => cmd.handle(cfg),
            Commands::GetDoor(cmd) => cmd.handle(cfg),
            Commands::SetDoor(cmd) => cmd.handle(cfg),
            Commands::SetDoorPasscodes(cmd) => cmd.handle(cfg),
            Commands::OpenDoor(cmd) => cmd.handle(cfg),
            Commands::GetStatus(cmd) => cmd.handle(cfg),
            Commands::GetCards(cmd) => cmd.handle(cfg),
            Commands::GetCard(cmd) => cmd.handle(cfg),
            Commands::GetCardAtIndex(cmd) => cmd.handle(cfg),
            Commands::PutCard(cmd) => cmd.handle(cfg),
            Commands::DeleteCard(cmd) => cmd.handle(cfg),
        }
    }

    pub fn list() -> Vec<&'static str> {
        vec![
            "find-controllers",
            "get-controller",
            "set-IPv4",
            "get-time",
            "get-listener",
            "set-listener",
            "get-door",
            "set-door",
            "set-door-passcodes",
            "open-door",
            "get-status",
            "get-cards",
            "get-card",
            "get-card-at-index",
            "put-card",
            "delete-card",
        ]
    }
}

impl FindControllerArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::find_controllers(cfg);
    }
}

impl GetControllerArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::get_controller(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
        );
    }
}

impl SetIPv4Args {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::set_IPv4(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
            self.address,
            self.netmask,
            self.gateway,
        );
    }
}

impl GetTimeArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::get_time(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
        );
    }
}

impl SetTimeArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        let datetime = Local::now().naive_local();

        commands::set_time(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
            datetime,
        );
    }
}

impl GetListenerArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::get_listener(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
        );
    }
}

impl SetListenerArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::set_listener(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
            self.address,
            self.interval,
        );
    }
}

impl GetDoorArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::get_door(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
            self.door,
        );
    }
}

impl SetDoorArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::set_door(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
            self.door,
            self.mode,
            self.delay,
        );
    }
}

impl SetDoorPasscodesArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::set_door_passcodes(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
            self.door,
            &self.passcodes,
        );
    }
}

impl OpenDoorArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::open_door(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
            self.door,
        );
    }
}

impl GetStatusArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::get_status(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
        );
    }
}

impl GetCardsArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::get_cards(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
        );
    }
}

impl GetCardArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::get_card(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
            self.card,
        );
    }
}

impl GetCardAtIndexArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::get_card_at_index(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
            self.index,
        );
    }
}

impl PutCardArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::put_card(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
            self.card,
            self.start_date,
            self.end_date,
            self.doors.as_str(),
            self.pin,
            self.firstcard.clone(),
        );
    }
}

impl DeleteCardArgs {
    pub fn handle(&self, cfg: uhppoted::Config) {
        commands::delete_card(
            cfg,
            self.controller.controller,
            self.controller.dest,
            self.controller.transport(),
            self.card,
        );
    }
}
