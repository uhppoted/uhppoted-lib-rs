use super::encode::Msg;

pub fn dump(packet: &Msg) {
    let debug = true;
    // let debug = if let Ok(guard) = DEBUG.read() {
    //     *guard
    // } else {
    //     false
    // };

    if debug {
        for i in 0..4 {
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

            println!("   {offset:08x}  {p}  {q}");
        }

        println!();
    }
}
