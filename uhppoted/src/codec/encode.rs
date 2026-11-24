use chrono::{NaiveDate, NaiveDateTime};
use std::net::{Ipv4Addr, SocketAddrV4};

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

pub fn set_listener_request(controller: u32, address: SocketAddrV4, interval: u8) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x90;

    pack_uint32(controller, &mut packet, 4)?;
    pack_IPv4(*address.ip(), &mut packet, 8)?;
    pack_uint16(address.port(), &mut packet, 12)?;
    pack_uint8(interval, &mut packet, 14)?;

    Ok(packet)
}

pub fn get_door_request(controller: u32, door: u8) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x82;

    pack_uint32(controller, &mut packet, 4)?;
    pack_uint8(door, &mut packet, 8)?;

    Ok(packet)
}

pub fn set_door_request(controller: u32, door: u8, mode: u8, delay: u8) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x80;

    pack_uint32(controller, &mut packet, 4)?;
    pack_uint8(door, &mut packet, 8)?;
    pack_uint8(mode, &mut packet, 9)?;
    pack_uint8(delay, &mut packet, 10)?;

    Ok(packet)
}

pub fn set_door_passcodes_request(
    controller: u32,
    door: u8,
    passcode1: u32,
    passcode2: u32,
    passcode3: u32,
    passcode4: u32,
) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x8c;

    pack_uint32(controller, &mut packet, 4)?;
    pack_uint8(door, &mut packet, 8)?;

    if passcode1 <= 999999 {
        pack_uint32(passcode1, &mut packet, 12)?;
    }

    if passcode2 <= 999999 {
        pack_uint32(passcode2, &mut packet, 16)?;
    }

    if passcode3 <= 999999 {
        pack_uint32(passcode3, &mut packet, 20)?;
    }

    if passcode4 < 999999 {
        pack_uint32(passcode4, &mut packet, 24)?;
    }

    Ok(packet)
}

pub fn open_door_request(controller: u32, door: u8) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x40;

    pack_uint32(controller, &mut packet, 4)?;
    pack_uint8(door, &mut packet, 8)?;

    Ok(packet)
}

pub fn get_status_request(controller: u32) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x20;

    pack_uint32(controller, &mut packet, 4)?;

    Ok(packet)
}

pub fn get_cards_request(controller: u32) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x58;

    pack_uint32(controller, &mut packet, 4)?;

    Ok(packet)
}

pub fn get_card_request(controller: u32, card: u32) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x5a;

    pack_uint32(controller, &mut packet, 4)?;
    pack_uint32(card, &mut packet, 8)?;

    Ok(packet)
}

pub fn get_card_at_index_request(controller: u32, index: u32) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x5c;

    pack_uint32(controller, &mut packet, 4)?;
    pack_uint32(index, &mut packet, 8)?;

    Ok(packet)
}

#[allow(non_snake_case)]
pub fn put_card_request(
    controller: u32,
    card: u32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    door1: u8,
    door2: u8,
    door3: u8,
    door4: u8,
    PIN: u32,
    firstcard: u8,
) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x50;

    pack_uint32(controller, &mut packet, 4)?;
    pack_uint32(card, &mut packet, 8)?;
    pack_date(start_date, &mut packet, 12)?;
    pack_date(end_date, &mut packet, 16)?;
    pack_uint8(door1, &mut packet, 20)?;
    pack_uint8(door2, &mut packet, 21)?;
    pack_uint8(door3, &mut packet, 22)?;
    pack_uint8(door4, &mut packet, 23)?;
    pack_uint24(PIN, &mut packet, 24)?;
    pack_uint8(firstcard, &mut packet, 27)?;

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

fn pack_uint24(v: u32, packet: &mut Msg, offset: usize) -> Result<()> {
    let bytes = v.to_le_bytes();

    packet[offset + 0] = bytes[0];
    packet[offset + 1] = bytes[1];
    packet[offset + 2] = bytes[2];

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

fn pack_date(v: NaiveDate, packet: &mut Msg, offset: usize) -> Result<()> {
    let s = v.format("%Y%m%d");
    let bcd = string2bcd!(s.to_string(), 4);

    packet[offset..offset + 4].clone_from_slice(&bcd);
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
