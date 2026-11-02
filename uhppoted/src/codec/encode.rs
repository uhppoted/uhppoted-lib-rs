use std::net::Ipv4Addr;

use super::Msg;
use super::Result;

const MAGIC: u32 = 0x55aaaa55;

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
    pack_ipv4(address, &mut packet, 8)?;
    pack_ipv4(netmask, &mut packet, 12)?;
    pack_ipv4(gateway, &mut packet, 16)?;
    pack_uint32(MAGIC, &mut packet, 20)?;

    Ok(packet)
}

fn pack_uint32(v: u32, packet: &mut Msg, offset: usize) -> Result<()> {
    let bytes = v.to_le_bytes();

    packet[offset..offset + 4].clone_from_slice(&bytes);
    Ok(())
}

fn pack_ipv4(v: Ipv4Addr, packet: &mut Msg, offset: usize) -> Result<()> {
    let addr = v.octets();

    packet[offset..offset + 4].clone_from_slice(&addr);
    Ok(())
}

#[cfg(test)]
mod generated_tests {
    use super::*;

    include!("generated/encode_tests.rs");
}
