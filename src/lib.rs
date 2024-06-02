use bytes::BytesMut;
use dashmap::DashMap;
use enum_dispatch::enum_dispatch;

mod decode;
mod encode;
pub mod network;
pub mod process;
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
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error>;
}

#[derive(Debug)]
pub struct Data {
    pub(crate) string_data: DashMap<String, Resp>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            string_data: DashMap::new(),
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}
