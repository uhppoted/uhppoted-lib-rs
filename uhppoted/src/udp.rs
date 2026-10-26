use std::io::ErrorKind::{Interrupted, TimedOut, WouldBlock};
use std::net::UdpSocket;
use std::rc::Rc;
use std::time::{Duration, Instant};

use super::Config;
use super::Msg;
use super::Result;
use super::error::ErrorKind;
use super::net::dump;

const WRITE_TIMEOUT: Duration = Duration::from_millis(250);

pub fn broadcast(cfg: &Config, request: &Msg, timeout: Duration) -> Result<Vec<Msg>> {
    // ... validate
    if cfg.bind == cfg.broadcast {
        return Err(ErrorKind::InvalidAddress.into());
    }

    let deadline = Instant::now() + timeout;

    // ... send-to-self
    //
    // If the bind port is 0, retries (up to 5 times) until the OS binds to a port that is not the
    // destination port. The interim sockets are retained to prevent the OS from reusing the invalid
    // port, which it will happily do if the socket is closed immediately.
    //
    // It should really match on both address and port but multiple network cards, etc make it complicated
    // and fragile.
    let mut sockets = Vec::<Rc<UdpSocket>>::new();
    let mut socket: Rc<UdpSocket>;
    let mut retries = 1;

    loop {
        socket = Rc::new(UdpSocket::bind(cfg.bind)?);
        sockets.push(Rc::clone(&socket));

        if socket.local_addr()?.port() != cfg.broadcast.port() {
            break;
        }

        if cfg.bind.port() != 0 {
            return Err(ErrorKind::BindToReservedPort.into());
        }

        if retries >= 5 {
            return Err(ErrorKind::BoundToReservedPort.into());
        }

        retries += 1;
    }

    socket.set_write_timeout(Some(WRITE_TIMEOUT))?;
    socket.set_broadcast(true)?;
    socket.send_to(request, cfg.broadcast)?;

    dump(
        "UDP",
        &format!("sent {} bytes to {}", 64, cfg.broadcast),
        request,
    );

    let mut replies = Vec::<Msg>::new();
    let mut buffer = [0u8; 1024];

    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());

        if remaining.is_zero() {
            break;
        }

        socket.set_read_timeout(Some(remaining))?;

        match socket.recv_from(&mut buffer) {
            Ok((64, from)) => {
                let packet: Msg = buffer[..64].try_into().unwrap();

                dump(
                    "UDP",
                    &format!("received {} bytes from {}", 64, from),
                    &packet,
                );

                replies.push(packet);
            }

            Ok((_, _)) => continue,

            Err(e) => match e.kind() {
                WouldBlock => continue,
                TimedOut => continue,
                Interrupted => continue,
                _ => return Err(e.into()),
            },
        }
    }

    Ok(replies)
}
