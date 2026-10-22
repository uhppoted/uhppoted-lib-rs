use std::io::ErrorKind;
use std::net::UdpSocket;
use std::time::{Duration, Instant};

use super::Config;
use super::Result;
use super::encode::Msg;
use super::net::dump;

const WRITE_TIMEOUT: Duration = Duration::from_millis(1000);

pub fn broadcast(cfg: &Config, request: &Msg) -> Result<Vec<Msg>> {
    let socket = UdpSocket::bind(cfg.bind.as_str())?;
    let timeout = Duration::from_millis(1250);

    dump(
        "UDP",
        &format!("sent {} bytes to {}", 64, "192.168.1.100:60000"),
        request,
    );

    socket.set_write_timeout(Some(WRITE_TIMEOUT))?;
    socket.set_broadcast(true)?;
    socket.send_to(request, cfg.broadcast.as_str())?;

    read_all(socket, timeout)
}

fn read_all(socket: UdpSocket, timeout: Duration) -> Result<Vec<Msg>> {
    let deadline = Instant::now() + timeout;
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
                dump(
                    "UDP",
                    &format!("received {} bytes from {}", 64, from),
                    &buffer[..64].try_into().unwrap(),
                );

                replies.push(buffer[..64].try_into().unwrap());
            }

            Ok((_, _)) => continue,

            Err(e) => match e.kind() {
                ErrorKind::WouldBlock => continue,
                ErrorKind::TimedOut => continue,
                ErrorKind::Interrupted => continue,
                _ => return Err(e.into()),
            },
        }
    }

    Ok(replies)
}
