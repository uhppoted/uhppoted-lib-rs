use chrono::{NaiveDate, NaiveDateTime};
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};

#[derive(Clone, Debug, PartialEq)]
pub struct ControllerRecord {
    pub controller: u32,
    pub ip_address: Ipv4Addr,
    pub subnet_mask: Ipv4Addr,
    pub gateway: Ipv4Addr,
    #[allow(non_snake_case)]
    pub MAC_address: String,
    pub version: String,
    pub date: NaiveDate,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListenerRecord {
    pub address: SocketAddrV4,
    pub interval: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatusRecord {
    pub system_datetime: NaiveDateTime,
    pub system_error: u8,
    pub system_special_info: u8,
    pub doors: Doors,
    pub alarms: Alarms,
    pub event: Option<Event>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Doors {
    pub open: HashMap<u8, bool>,
    pub button: HashMap<u8, bool>,
    pub unlocked: HashMap<u8, bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Alarms {
    pub flags: u8,
    pub fire: bool,
    pub lock_forced: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub index: u32,
    pub event_type: u8,
    pub access_granted: bool,
    pub door: u8,
    pub direction: u8,
    pub card: u32,
    pub timestamp: Option<NaiveDateTime>,
    pub reason: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DoorRecord {
    pub door: u8,
    pub mode: u8,
    pub delay: u8,
}
