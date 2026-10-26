use super::Msg;
use super::Result;

pub trait Decode: Sized {
    fn decode(packet: &Msg) -> Result<Self>;
}
