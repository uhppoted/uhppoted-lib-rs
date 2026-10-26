use chrono::NaiveDate;
use regex::Regex;
use std::net::Ipv4Addr;

use super::Msg;
use super::Result;
use super::error::Error;

#[derive(Clone, Debug)]
pub struct GetControllerResponse {
    pub controller: u32,
    pub ip_address: Ipv4Addr,
    pub subnet_mask: Ipv4Addr,
    pub gateway: Ipv4Addr,
    pub mac_address: String,
    pub version: String,
    pub date: NaiveDate,
}

pub fn get_controller_response(packet: &Msg) -> Result<GetControllerResponse> {
    if packet.len() != 64 {
        return Err(format!("Invalid reply packet length ({})", packet.len()))?;
    }

    // Ref. v6.62 firmware event
    if packet[0] != 0x17 && (packet[0] != 0x19 || packet[1] != 0x20) {
        return Err(format!(
            "Invalid reply start of message byte ({:02x})",
            packet[0]
        ))?;
    }

    if packet[1] != 0x94 {
        return Err(format!("Invalid reply function code ({:02x})", packet[1]))?;
    }

    let response = GetControllerResponse {
        controller: unpack_uint32(packet, 4)?,
        ip_address: unpack_ipv4(packet, 8)?,
        subnet_mask: unpack_ipv4(packet, 12)?,
        gateway: unpack_ipv4(packet, 16)?,
        mac_address: unpack_mac(packet, 20)?,
        version: unpack_version(packet, 26)?,
        date: unpack_date(packet, 28)?,
    };

    return Ok(response);
}

#[macro_export]
macro_rules! bcd2string {
    ($slice:expr) => {{
        let s: String = $slice.iter().map(|b| format!("{:02x}", b)).collect();

        let v = if Regex::new(r"^[0-9]*$").unwrap().is_match(&s) {
            Ok(s)
        } else {
            Err(Error::from(format!("invalid BCD value {s}")))
        };

        v.unwrap()
    }};
}

fn unpack_uint32(packet: &Msg, offset: usize) -> Result<u32> {
    let mut bytes: [u8; 4] = [0; 4];

    bytes.clone_from_slice(&packet[offset..offset + 4]);

    return Ok(u32::from_le_bytes(bytes));
}

fn unpack_ipv4(packet: &Msg, offset: usize) -> Result<Ipv4Addr> {
    let u0 = packet[offset];
    let u1 = packet[offset + 1];
    let u2 = packet[offset + 2];
    let u3 = packet[offset + 3];

    return Ok(Ipv4Addr::new(u0, u1, u2, u3));
}

fn unpack_mac(packet: &Msg, offset: usize) -> Result<String> {
    let mac = format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        packet[offset],
        packet[offset + 1],
        packet[offset + 2],
        packet[offset + 3],
        packet[offset + 4],
        packet[offset + 5]
    );

    return Ok(mac);
}

fn unpack_version(packet: &Msg, offset: usize) -> Result<String> {
    let major = packet[offset];
    let minor = packet[offset + 1];

    return Ok(format!("v{major:x}.{minor:02x}").to_string());
}

fn unpack_date(packet: &Msg, offset: usize) -> Result<NaiveDate> {
    let slice: &[u8; 4] = packet[offset..offset + 4].try_into().unwrap();
    let s = bcd2string!(slice);

    match NaiveDate::parse_from_str(&s, "%Y%m%d") {
        Ok(date) => return Ok(date),
        Err(_) => Err(Error::from(format!("invalid date string {}", s))),
    }
}
