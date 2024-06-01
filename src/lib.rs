use bytes::BytesMut;
use enum_dispatch::enum_dispatch;

mod decode;
mod encode;
pub mod network;
mod process;
pub mod resp;
use crate::process::string::{get::GetCommandPara, set::SetCommandPara, StringCommand};
use crate::process::CommandGroup;
pub use resp::*;

pub trait RespDecoder: Sized {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
}

#[enum_dispatch]
pub trait RespEncoder {
    fn encode(self) -> Result<Vec<u8>, anyhow::Error>;
}

#[enum_dispatch]
pub trait Processor {
    fn process(&self) -> Result<Box<dyn RespEncoder>, anyhow::Error>;
}
