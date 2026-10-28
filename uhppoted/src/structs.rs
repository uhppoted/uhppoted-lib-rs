use chrono::NaiveDate;
use std::net::Ipv4Addr;

#[derive(Clone, Debug)]
pub struct ControllerRecord {
    pub controller: u32,
    pub ip_address: Ipv4Addr,
    pub subnet_mask: Ipv4Addr,
    pub gateway: Ipv4Addr,
    pub mac_address: String,
    pub version: String,
    pub date: NaiveDate,
}
