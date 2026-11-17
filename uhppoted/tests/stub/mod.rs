use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpListener, UdpSocket};
use std::sync::{Arc, OnceLock};
use std::thread;

mod generated {
    include!("../generated/messages.rs");
}

pub(crate) type Packet = [u8; 64];

#[derive(Clone, Debug)]
pub struct TestStub {
    pub cfg: uhppoted::Config,
}

pub fn broadcast() -> &'static TestStub {
    static STUB: OnceLock<TestStub> = OnceLock::new();

    STUB.get_or_init(|| {
        let cfg = uhppoted::Config {
            bind: "0.0.0.0:0".parse().unwrap(),
            broadcast: "255.255.255.255:60005".parse().unwrap(),
            listen: "0.0.0.0:60001".parse().unwrap(),
        };

        let addr: SocketAddrV4 = "0.0.0.0:60005".parse().unwrap();
        let socket = UdpSocket::bind(addr).unwrap_or_else(|e| panic!("failed to bind UDP socket to {addr}: {e}"));
        let messages = Arc::new(generated::message());

        thread::spawn(move || {
            let mut buffer = [0u8; 1024];

            loop {
                match socket.recv_from(&mut buffer) {
                    Ok((64, src)) => {
                        let packet: [u8; 64] = buffer[..64].try_into().unwrap();
                        let messages = Arc::clone(&messages);
                        let tx_socket = socket.try_clone().expect("failed to clone UDP socket");

                        thread::spawn(move || {
                            if let Some(replies) = messages.get(&packet[..]) {
                                for reply in replies {
                                    tx_socket.send_to(reply, src).expect("error sending reply");
                                }
                            } else if packet[1] != 0x96 {
                                dump("OOPS", "unrecognised packet", &packet);
                            }
                        });
                    }

                    Ok((n, src)) => {
                        eprintln!("UDP receive {n} bytes from {src}");
                    }

                    Err(e) => {
                        eprintln!("UDP receive error: {e}");
                        break;
                    }
                }
            }
        });

        TestStub { cfg }
    })
}

pub fn udp() -> &'static TestStub {
    static STUB: OnceLock<TestStub> = OnceLock::new();

    STUB.get_or_init(|| {
        let cfg = uhppoted::Config {
            bind: "0.0.0.0:0".parse().unwrap(),
            broadcast: "255.255.255.255:60000".parse().unwrap(),
            listen: "0.0.0.0:60001".parse().unwrap(),
        };

        let addr: SocketAddrV4 = "0.0.0.0:60004".parse().unwrap();
        let socket = UdpSocket::bind(addr).unwrap_or_else(|e| panic!("failed to bind UDP socket to {addr}: {e}"));
        let messages = Arc::new(generated::message());

        thread::spawn(move || {
            let mut buffer = [0u8; 1024];

            loop {
                match socket.recv_from(&mut buffer) {
                    Ok((64, src)) => {
                        let packet: [u8; 64] = buffer[..64].try_into().unwrap();
                        let messages = Arc::clone(&messages);
                        let tx_socket = socket.try_clone().expect("failed to clone UDP socket");

                        thread::spawn(move || {
                            if let Some(replies) = messages.get(&packet[..]) {
                                for reply in replies {
                                    tx_socket.send_to(reply, src).expect("error sending reply");
                                }
                            } else if packet[1] != 0x96 {
                                dump("OOPS", "unrecognised packet", &packet);
                            }
                        });
                    }

                    Ok((n, src)) => {
                        eprintln!("UDP receive {n} bytes from {src}");
                    }

                    Err(e) => {
                        eprintln!("UDP receive error: {e}");
                        break;
                    }
                }
            }
        });

        TestStub { cfg }
    })
}

pub fn tcp() -> &'static TestStub {
    static STUB: OnceLock<TestStub> = OnceLock::new();

    STUB.get_or_init(|| {
        let cfg = uhppoted::Config {
            bind: "0.0.0.0:0".parse().unwrap(),
            broadcast: "255.255.255.255:60000".parse().unwrap(),
            listen: "0.0.0.0:60001".parse().unwrap(),
        };

        let addr: SocketAddrV4 = "0.0.0.0:60003".parse().unwrap();
        let listener = TcpListener::bind(addr).unwrap_or_else(|e| panic!("failed to bind TCP listener to {addr}: {e}"));
        let messages = Arc::new(generated::message());

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        let messages = Arc::clone(&messages);

                        thread::spawn(move || {
                            let mut buffer = [0u8; 1024];

                            loop {
                                match stream.read(&mut buffer) {
                                    Ok(0) => break,
                                    Ok(64) => {
                                        let packet: [u8; 64] = buffer[..64].try_into().unwrap();

                                        if let Some(replies) = messages.get(&packet[..]) {
                                            for reply in replies {
                                                if let Err(e) = stream.write_all(reply) {
                                                    eprintln!("TCP write error: {e}");
                                                    break;
                                                }
                                            }
                                        } else if packet[1] != 0x96 {
                                            dump("OOPS", "unrecognised packet", &packet);
                                        }
                                    }
                                    Ok(n) => {
                                        eprintln!("TCP receive {n} bytes (expected 64)");
                                    }
                                    Err(e) => {
                                        eprintln!("TCP read error: {e}");
                                        break;
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("TCP accept error: {e}");
                    }
                }
            }
        });

        TestStub { cfg }
    })
}

pub fn dump(tag: &str, description: &str, packet: &Packet) {
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
        "{}  {}\n{}\n{}\n{}\n{}\n",
        tag, description, hex[0], hex[1], hex[2], hex[3]
    );

    eprintln!("{}", msg)
}
