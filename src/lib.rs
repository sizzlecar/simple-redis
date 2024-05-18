use enum_dispatch::enum_dispatch;

mod decode;
mod encode;
mod process;
pub mod resp;
use crate::process::string::{get::GetCommandPara, set::SetCommandPara, StringCommand};
use crate::process::CommandGroup;
pub use resp::*;

#[enum_dispatch]
pub trait Decoder {
    fn decode(self) -> Result<Box<dyn Processor>, anyhow::Error>;
}

#[enum_dispatch]
pub trait Encoder {
    fn encode(self) -> Result<Vec<u8>, anyhow::Error>;
}

#[enum_dispatch]
pub trait Processor {
    fn process(self) -> Result<Box<dyn Encoder>, anyhow::Error>;
}
