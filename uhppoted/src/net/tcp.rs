use std::io::ErrorKind::{Interrupted, TimedOut, WouldBlock};
use std::io::{Read, Write};
use std::net::{SocketAddr, SocketAddrV4, TcpStream};
use std::time::{Duration, Instant};

use super::Config;
use super::Msg;
use super::Result;
use super::error::ErrorKind;
use super::net::dump;

// NTS: Rust's TCP implementation is beyond braindead :-(
pub fn send_to(_cfg: &Config, dest: SocketAddrV4, request: &Msg, timeout: Duration) -> Result<Msg> {
    // NTS: TCPStream always binds to an ephemeral port
    // if cfg.bind == dest {
    //     return Err(ErrorKind::InvalidAddress.into());
    // }

    let addr = SocketAddr::V4(dest);
    let deadline = Instant::now() + timeout;
    let remaining = || deadline.saturating_duration_since(Instant::now());

    // ... send-to-self
    //
    // If the bind port is 0, retries (up to 5 times) until the OS binds to a port that is not the
    // destination port. The interim sockets are retained to prevent the OS from reusing the invalid
    // port, which it will happily do if the socket is closed immediately.
    //
    // It should really match on both address and port but multiple network cards, etc make it complicated
    // and fragile.
    let mut sockets: Vec<TcpStream> = Vec::new();

    let mut socket = loop {
        let _remaining = remaining();
        if _remaining.is_zero() {
            return Err(ErrorKind::Timeout.into());
        }

        let sock = TcpStream::connect_timeout(&addr, _remaining)?;
        if sock.local_addr()?.port() != dest.port() {
            break sock;
        }

        // if cfg.bind.port() != 0 {
        //     return Err(ErrorKind::BindToReservedPort.into());
        // }

        sockets.push(sock);
        if sockets.len() >= 5 {
            return Err(ErrorKind::BoundToReservedPort.into());
        }
    };

    let _remaining = remaining();
    if _remaining.is_zero() {
        return Err(ErrorKind::Timeout.into());
    }

    socket.set_write_timeout(Some(_remaining))?;
    socket.write_all(request)?;

    dump("TCP", &format!("sent {} bytes to {}", 64, dest), request);

    let mut buffer = [0u8; 1024];

    loop {
        let _remaining = remaining();
        if _remaining.is_zero() {
            return Err(ErrorKind::Timeout.into());
        }

        socket.set_read_timeout(Some(_remaining))?;

        match socket.read(&mut buffer) {
            Ok(64) => {
                let packet: Msg = buffer[..64].try_into().unwrap();

                dump(
                    "TCP",
                    &format!("received {} bytes from {}", 64, socket.peer_addr()?),
                    &packet,
                );

                return Ok(packet);
            }

            Ok(0) => {
                return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof).into());
            }

            Ok(_) => continue,

            Err(e) => match e.kind() {
                WouldBlock => continue,
                TimedOut => continue,
                Interrupted => continue,
                _ => return Err(e.into()),
            },
        };
    }
}
