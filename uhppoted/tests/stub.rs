use std::net::{SocketAddrV4, UdpSocket};
use std::sync::OnceLock;
use std::thread;

mod generated {
    include!("generated/messages.rs");
}

#[derive(Clone, Debug)]
pub struct TestStub {
    pub cfg: uhppoted::Config,
}

pub fn start() -> &'static TestStub {
    static STUB: OnceLock<TestStub> = OnceLock::new();

    STUB.get_or_init(|| {
        let cfg = uhppoted::Config {
            bind: "0.0.0.0:0".parse().unwrap(),
            broadcast: "255.255.255.255:60005".parse().unwrap(),
            listen: "0.0.0.0:60001".parse().unwrap(),
        };

        let addr: SocketAddrV4 = "0.0.0.0:60005".parse().unwrap();

        thread::spawn(move || {
            let socket = UdpSocket::bind(addr).unwrap_or_else(|e| panic!("failed to bind UDP socket to {addr}: {e}"));
            let messages = generated::message();

            loop {
                let mut buffer = [0u8; 1024];

                match socket.recv_from(&mut buffer) {
                    Ok((64, src)) => {
                        if let Some(replies) = messages.get(&buffer[..64]) {
                            for reply in replies {
                                socket.send_to(reply, src).expect("error sending reply");
                            }
                        }
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
