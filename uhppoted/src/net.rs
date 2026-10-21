use log::debug;

use super::encode::Msg;

pub fn dump(tag: &str, description: &str, packet: &Msg) {
    let hex: Vec<String> = (0..4)
        .map(|i| {
            let offset = i * 16;
            let u = &packet[offset..offset + 8];
            let v = &packet[offset + 8..offset + 16];

            let p = format!(
                "{:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
                u[0], u[1], u[2], u[3], u[4], u[5], u[6], u[7]
            );

            let q = format!(
                "{:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
                v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]
            );

            format!("{:08x}  {}  {}", offset, p, q)
        })
        .collect();

    let msg = format!(
        "{}  {}\n{}\n{}\n{}\n{}",
        tag, description, hex[0], hex[1], hex[2], hex[3]
    );

    debug!("{}", msg)
}
