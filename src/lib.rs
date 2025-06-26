use bytes::BytesMut;
use dashmap::DashMap;
use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};

mod decode;
mod encode;
pub mod network;
pub mod process;
pub mod resp;

use crate::process::string::get::GetCommandPara;
pub use resp::*;

pub trait RespDecoder: Sized {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
}

pub trait RespEncoder {
    fn encode(self) -> Result<Vec<u8>, anyhow::Error>;
}

pub trait Processor {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error>;
}

#[derive(Debug)]
pub struct Data {
    pub(crate) string_data: DashMap<String, Resp>,
    pub(crate) hash_data: DashMap<String, HashMap<String, String>>,
    pub(crate) list_data: DashMap<String, VecDeque<String>>,
    pub(crate) set_data: DashMap<String, HashSet<String>>,
    pub(crate) sorted_set_data: DashMap<String, BTreeMap<String, f64>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            string_data: DashMap::new(),
            hash_data: DashMap::new(),
            list_data: DashMap::new(),
            set_data: DashMap::new(),
            sorted_set_data: DashMap::new(),
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}
