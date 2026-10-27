use super::Msg;
use super::Result;

pub fn get_controller_request(controller: u32) -> Result<Msg> {
    let mut packet = [0x00; 64];

    packet[0] = 0x17;
    packet[1] = 0x94;

    pack_uint32(controller, &mut packet, 4)?;

    Ok(packet)
}

fn pack_uint32(v: u32, packet: &mut Msg, offset: usize) -> Result<()> {
    let bytes = v.to_le_bytes();

    packet[offset..offset + 4].clone_from_slice(&bytes);
    Ok(())
}

#[cfg(test)]
mod generated_tests {
    use super::*;

    include!("generated/encode_tests.rs");
}
