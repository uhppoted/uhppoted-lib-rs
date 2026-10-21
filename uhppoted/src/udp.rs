use std::net::UdpSocket;
use std::time::Duration;

use super::Config;
use super::Result;
use super::encode::Msg;
use super::net::dump;

const READ_TIMEOUT: Duration = Duration::from_millis(5000);
const WRITE_TIMEOUT: Duration = Duration::from_millis(1000);
// const TIMEOUT: Duration = Duration::from_millis(2500);

pub fn broadcast(cfg: &Config, request: &Msg) -> Result<Vec<Msg>> {
    let socket = UdpSocket::bind(cfg.bind.as_str())?;

    dump(
        "UDP",
        &format!("sent {} bytes to {}", 64, "192.168.1.100:60000"),
        request,
    );

    socket.set_write_timeout(Some(WRITE_TIMEOUT))?;
    socket.set_read_timeout(Some(READ_TIMEOUT))?;
    socket.set_broadcast(true)?;
    socket.send_to(request, cfg.broadcast.as_str())?;

    read_all(socket)
}

fn read_all(_socket: UdpSocket) -> Result<Vec<Msg>> {
    let replies = Vec::<Msg>::new();

    // let replies = RwLock::<Vec<Msg>>::new(vec![]);
    //
    // let f = future::timeout(Duration::from_millis(2500), async {
    //     let sock = async_std::net::UdpSocket::from(socket);
    //     let mut buffer = [0u8; 1024];
    //
    //     let result: Result<()> = loop {
    //         match sock.recv(&mut buffer).await {
    //             Ok(64) => {
    //                 dump(buffer[..64].try_into()?);
    //                 replies.write().unwrap().push(buffer[..64].try_into()?);
    //             }
    //
    //             Err(e) => break Err(error::Error::from(e)),
    //             _ => continue,
    //         }
    //     };
    //
    //     return result;
    // });
    //
    // return async {
    //     match f.await {
    //         Ok(v) => match v {
    //             Ok(_) => return Ok(replies.read().unwrap().to_vec()),
    //             Err(e) => return Err(error::Error::from(e)),
    //         },
    //
    //         Err(_) => return Ok(replies.read().unwrap().to_vec()),
    //     }
    // }
    // .await;

    Ok(replies)
}
