use chrono::NaiveDate;
use std::net::Ipv4Addr;

#[derive(PartialEq, Clone, Debug)]
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
