use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use regex::Regex;
use std::net::Ipv4Addr;

pub mod codec;

use super::Msg;
use super::Result;
use super::error::Error;

use crate::decode::codec::Decode;

macro_rules! validate {
    ($packet:expr, $fn: expr) => {
        if $packet.len() != 64 {
            Err(format!("Invalid reply packet length ({})", $packet.len()))?
        }

        // Ref. v6.62 firmware event
        if $packet[0] != 0x17 && ($packet[0] != 0x19 || $packet[1] != 0x20) {
            Err(format!("Invalid reply start of message byte ({:02x})", $packet[0]))?
        }

        if $packet[1] != $fn {
            Err(format!("Invalid reply function code ({:02x})", $packet[1]))?
        }
    };
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetControllerResponse {
    pub controller: u32,
    pub ip_address: Ipv4Addr,
    pub subnet_mask: Ipv4Addr,
    pub gateway: Ipv4Addr,
    #[allow(non_snake_case)]
    pub MAC_address: String,
    pub version: String,
    pub date: NaiveDate,
}

impl codec::Decode for GetControllerResponse {
    fn decode(packet: &Msg) -> Result<GetControllerResponse> {
        validate!(packet, 0x94);

        let response = GetControllerResponse {
            controller: unpack_uint32(packet, 4)?,
            ip_address: unpack_IPv4(packet, 8)?,
            subnet_mask: unpack_IPv4(packet, 12)?,
            gateway: unpack_IPv4(packet, 16)?,
            MAC_address: unpack_MAC(packet, 20)?,
            version: unpack_version(packet, 26)?,
            date: unpack_date(packet, 28)?,
        };

        Ok(response)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetIPv4Response {
    pub controller: u32,
    pub ok: bool,
}

impl codec::Decode for SetIPv4Response {
    fn decode(packet: &Msg) -> Result<SetIPv4Response> {
        validate!(packet, 0x96);

        let response = SetIPv4Response {
            controller: unpack_uint32(packet, 4)?,
            ok: unpack_bool(packet, 8)?,
        };

        Ok(response)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetTimeResponse {
    pub controller: u32,
    pub date_time: NaiveDateTime,
}

impl codec::Decode for GetTimeResponse {
    fn decode(packet: &Msg) -> Result<GetTimeResponse> {
        validate!(packet, 0x32);

        let response = GetTimeResponse {
            controller: unpack_uint32(packet, 4)?,
            date_time: unpack_datetime(packet, 8)?,
        };

        Ok(response)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetTimeResponse {
    pub controller: u32,
    pub date_time: NaiveDateTime,
}

impl codec::Decode for SetTimeResponse {
    fn decode(packet: &Msg) -> Result<SetTimeResponse> {
        validate!(packet, 0x30);

        let response = SetTimeResponse {
            controller: unpack_uint32(packet, 4)?,
            date_time: unpack_datetime(packet, 8)?,
        };

        Ok(response)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetListenerResponse {
    pub controller: u32,
    pub address: Ipv4Addr,
    pub port: u16,
    pub interval: u8,
}

impl codec::Decode for GetListenerResponse {
    fn decode(packet: &Msg) -> Result<GetListenerResponse> {
        validate!(packet, 0x92);

        let response = GetListenerResponse {
            controller: unpack_uint32(packet, 4)?,
            address: unpack_IPv4(packet, 8)?,
            port: unpack_uint16(packet, 12)?,
            interval: unpack_uint8(packet, 14)?,
        };

        Ok(response)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetListenerResponse {
    pub controller: u32,
    pub ok: bool,
}

impl codec::Decode for SetListenerResponse {
    fn decode(packet: &Msg) -> Result<SetListenerResponse> {
        validate!(packet, 0x90);

        let response = SetListenerResponse {
            controller: unpack_uint32(packet, 4)?,
            ok: unpack_bool(packet, 8)?,
        };

        Ok(response)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetStatusResponse {
    pub controller: u32,
    pub system_date: NaiveDate,
    pub system_time: NaiveTime,
    pub door_1_open: bool,
    pub door_2_open: bool,
    pub door_3_open: bool,
    pub door_4_open: bool,
    pub door_1_button: bool,
    pub door_2_button: bool,
    pub door_3_button: bool,
    pub door_4_button: bool,
    pub relays: u8,
    pub inputs: u8,
    pub system_error: u8,
    pub special_info: u8,
    pub event_index: u32,
    pub event_type: u8,
    pub event_access_granted: bool,
    pub event_door: u8,
    pub event_direction: u8,
    pub event_card: u32,
    pub event_timestamp: Option<NaiveDateTime>,
    pub event_reason: u8,
    pub sequence_no: u32,
}

impl codec::Decode for GetStatusResponse {
    fn decode(packet: &Msg) -> Result<GetStatusResponse> {
        validate!(packet, 0x20);

        let response = GetStatusResponse {
            controller: unpack_uint32(packet, 4)?,
            system_date: unpack_shortdate(packet, 51)?,
            system_time: unpack_time(packet, 37)?,
            door_1_open: unpack_bool(packet, 28)?,
            door_2_open: unpack_bool(packet, 29)?,
            door_3_open: unpack_bool(packet, 30)?,
            door_4_open: unpack_bool(packet, 31)?,
            door_1_button: unpack_bool(packet, 32)?,
            door_2_button: unpack_bool(packet, 33)?,
            door_3_button: unpack_bool(packet, 34)?,
            door_4_button: unpack_bool(packet, 35)?,
            relays: unpack_uint8(packet, 49)?,
            inputs: unpack_uint8(packet, 50)?,
            system_error: unpack_uint8(packet, 36)?,
            special_info: unpack_uint8(packet, 48)?,
            event_index: unpack_uint32(packet, 8)?,
            event_type: unpack_uint8(packet, 12)?,
            event_access_granted: unpack_bool(packet, 13)?,
            event_door: unpack_uint8(packet, 14)?,
            event_direction: unpack_uint8(packet, 15)?,
            event_card: unpack_uint32(packet, 16)?,
            event_timestamp: unpack_optional_datetime(packet, 20)?,
            event_reason: unpack_uint8(packet, 27)?,
            sequence_no: unpack_uint32(packet, 40)?,
        };

        Ok(response)
    }
}

pub fn get_controller_response(packet: &Msg) -> Result<GetControllerResponse> {
    GetControllerResponse::decode(packet)
}

#[allow(non_snake_case)]
pub fn set_IPv4_response(packet: &Msg) -> Result<SetIPv4Response> {
    SetIPv4Response::decode(packet)
}

pub fn get_time_response(packet: &Msg) -> Result<GetTimeResponse> {
    GetTimeResponse::decode(packet)
}

pub fn set_time_response(packet: &Msg) -> Result<SetTimeResponse> {
    SetTimeResponse::decode(packet)
}

pub fn get_listener_response(packet: &Msg) -> Result<GetListenerResponse> {
    GetListenerResponse::decode(packet)
}

pub fn set_listener_response(packet: &Msg) -> Result<SetListenerResponse> {
    SetListenerResponse::decode(packet)
}

pub fn get_status_response(packet: &Msg) -> Result<GetStatusResponse> {
    GetStatusResponse::decode(packet)
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

fn unpack_uint8(packet: &Msg, offset: usize) -> Result<u8> {
    Ok(packet[offset])
}

fn unpack_uint16(packet: &Msg, offset: usize) -> Result<u16> {
    let mut bytes: [u8; 2] = [0; 2];

    bytes.clone_from_slice(&packet[offset..offset + 2]);

    Ok(u16::from_le_bytes(bytes))
}

fn unpack_uint32(packet: &Msg, offset: usize) -> Result<u32> {
    let mut bytes: [u8; 4] = [0; 4];

    bytes.clone_from_slice(&packet[offset..offset + 4]);

    Ok(u32::from_le_bytes(bytes))
}

fn unpack_bool(packet: &Msg, offset: usize) -> Result<bool> {
    Ok(packet[offset] != 0x00)
}

#[allow(non_snake_case)]
fn unpack_IPv4(packet: &Msg, offset: usize) -> Result<Ipv4Addr> {
    let u0 = packet[offset];
    let u1 = packet[offset + 1];
    let u2 = packet[offset + 2];
    let u3 = packet[offset + 3];

    Ok(Ipv4Addr::new(u0, u1, u2, u3))
}

#[allow(non_snake_case)]
fn unpack_MAC(packet: &Msg, offset: usize) -> Result<String> {
    let mac = format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        packet[offset],
        packet[offset + 1],
        packet[offset + 2],
        packet[offset + 3],
        packet[offset + 4],
        packet[offset + 5]
    );

    Ok(mac)
}

fn unpack_version(packet: &Msg, offset: usize) -> Result<String> {
    let major = packet[offset];
    let minor = packet[offset + 1];

    Ok(format!("v{major:x}.{minor:02x}").to_string())
}

fn unpack_date(packet: &Msg, offset: usize) -> Result<NaiveDate> {
    let slice: &[u8; 4] = packet[offset..offset + 4].try_into().unwrap();
    let s = bcd2string!(slice);

    match NaiveDate::parse_from_str(&s, "%Y%m%d") {
        Ok(date) => Ok(date),
        Err(_) => Err(Error::from(format!("invalid date string {}", s))),
    }
}

fn unpack_shortdate(packet: &Msg, offset: usize) -> Result<NaiveDate> {
    let slice: &[u8; 3] = packet[offset..offset + 3].try_into().unwrap();
    let s = format!("20{}", bcd2string!(slice));

    match NaiveDate::parse_from_str(&s, "%Y%m%d") {
        Ok(date) => Ok(date),
        Err(_) => Err(Error::from(format!("invalid date string {}", s))),
    }
}

fn unpack_time(packet: &Msg, offset: usize) -> Result<NaiveTime> {
    let slice: &[u8; 3] = packet[offset..offset + 3].try_into().unwrap();
    let s: String = bcd2string!(slice);

    match NaiveTime::parse_from_str(&s, "%H%M%S") {
        Ok(time) => Ok(time),
        Err(_) => Err(Error::from(format!("invalid time string {}", s))),
    }
}

fn unpack_datetime(packet: &Msg, offset: usize) -> Result<NaiveDateTime> {
    let slice: &[u8; 7] = packet[offset..offset + 7].try_into().unwrap();
    let s: String = bcd2string!(slice);

    match NaiveDateTime::parse_from_str(&s, "%Y%m%d%H%M%S") {
        Ok(datetime) => Ok(datetime),
        Err(_) => Err(Error::from(format!("invalid date/time string {}", s))),
    }
}

fn unpack_optional_datetime(packet: &Msg, offset: usize) -> Result<Option<NaiveDateTime>> {
    let slice: &[u8; 7] = packet[offset..offset + 7].try_into().unwrap();
    let s: String = bcd2string!(slice);

    match NaiveDateTime::parse_from_str(&s, "%Y%m%d%H%M%S") {
        Ok(datetime) => Ok(Some(datetime)),
        Err(_) => Ok(None),
    }
}

#[cfg(test)]
mod generated_tests {
    use super::*;

    include!("generated/decode_tests.rs");
}
