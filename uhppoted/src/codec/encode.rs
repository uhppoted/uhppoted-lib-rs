use chrono::NaiveDateTime;
use std::net::Ipv4Addr;

use super::Msg;
use super::Result;
use super::error::Error;

const MAGIC: u32 = 0x55aaaa55;

#[macro_export]
macro_rules! string2bcd {
    ($bcd:expr, $size:expr) => {{
        let mut nibbles: [u8; 2 * $size] = [0; 2 * $size];
        let mut bytes: [u8; $size] = [0; $size];
        let mut ix = 0;

        if $bcd.len() % 2 != 0 {
            ix = 1;
        }

        for ch in $bcd.chars() {
            nibbles[ix] = bcd(ch)?;
            ix += 1;
        }

        for (i, pair) in nibbles.chunks(2).enumerate() {
            bytes[i] = (pair[0] << 4) | pair[1];
        }

        bytes
    }};
}

fn bcd(ch: char) -> Result<u8> {
    match ch {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        _ => Err(Error::from(format!("invalid BCD digit {ch}"))),
    }
}

pub fn get_controller_request(controller: u32) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x94;

    pack_uint32(controller, &mut packet, 4)?;

    Ok(packet)
}

#[allow(non_snake_case)]
pub fn set_IPv4_request(controller: u32, address: Ipv4Addr, netmask: Ipv4Addr, gateway: Ipv4Addr) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x96;

    pack_uint32(controller, &mut packet, 4)?;
    pack_IPv4(address, &mut packet, 8)?;
    pack_IPv4(netmask, &mut packet, 12)?;
    pack_IPv4(gateway, &mut packet, 16)?;
    pack_uint32(MAGIC, &mut packet, 20)?;

    Ok(packet)
}

pub fn get_time_request(controller: u32) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x32;

    pack_uint32(controller, &mut packet, 4)?;

    Ok(packet)
}

pub fn set_time_request(controller: u32, datetime: NaiveDateTime) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x30;

    pack_uint32(controller, &mut packet, 4)?;
    pack_datetime(datetime, &mut packet, 8)?;

    Ok(packet)
}

pub fn get_listener_request(controller: u32) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x92;

    pack_uint32(controller, &mut packet, 4)?;

    Ok(packet)
}

pub fn set_listener_request(controller: u32, address: Ipv4Addr, port: u16, interval: u8) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x90;

    pack_uint32(controller, &mut packet, 4)?;
    pack_IPv4(address, &mut packet, 8)?;
    pack_uint16(port, &mut packet, 12)?;
    pack_uint8(interval, &mut packet, 14)?;

    Ok(packet)
}

pub fn get_status_request(controller: u32) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x20;

    pack_uint32(controller, &mut packet, 4)?;

    Ok(packet)
}

fn pack_uint8(v: u8, packet: &mut Msg, offset: usize) -> Result<()> {
    packet[offset] = v;
    Ok(())
}

fn pack_uint16(v: u16, packet: &mut Msg, offset: usize) -> Result<()> {
    let bytes = v.to_le_bytes();

    packet[offset..offset + 2].clone_from_slice(&bytes);
    Ok(())
}

fn pack_uint32(v: u32, packet: &mut Msg, offset: usize) -> Result<()> {
    let bytes = v.to_le_bytes();

    packet[offset..offset + 4].clone_from_slice(&bytes);
    Ok(())
}

#[allow(non_snake_case)]
fn pack_IPv4(v: Ipv4Addr, packet: &mut Msg, offset: usize) -> Result<()> {
    let addr = v.octets();

    packet[offset..offset + 4].clone_from_slice(&addr);
    Ok(())
}

fn pack_datetime(v: NaiveDateTime, packet: &mut Msg, offset: usize) -> Result<()> {
    let s = v.format("%Y%m%d%H%M%S");
    let bcd = string2bcd!(s.to_string(), 7);

    packet[offset..offset + 7].clone_from_slice(&bcd);
    Ok(())
}

#[cfg(test)]
mod generated_tests {
    use super::*;

    include!("generated/encode_tests.rs");
}
