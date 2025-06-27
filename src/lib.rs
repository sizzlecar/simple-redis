use bytes::BytesMut;
use dashmap::DashMap;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

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
    // 过期时间存储，键 -> 过期时间戳（毫秒）
    pub(crate) expiry_data: DashMap<String, u64>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            string_data: DashMap::new(),
            hash_data: DashMap::new(),
            list_data: DashMap::new(),
            set_data: DashMap::new(),
            sorted_set_data: DashMap::new(),
            expiry_data: DashMap::new(),
        }
    }

    // 获取当前时间戳（毫秒）
    pub fn current_timestamp_millis(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    // 检查键是否过期
    pub fn is_expired(&self, key: &str) -> bool {
        if let Some(expiry) = self.expiry_data.get(key) {
            let now = self.current_timestamp_millis();
            now >= *expiry
        } else {
            false
        }
    }

    // 清理过期的键
    pub fn cleanup_expired(&self) {
        let now = self.current_timestamp_millis();
        let expired_keys: Vec<String> = self
            .expiry_data
            .iter()
            .filter(|entry| now >= *entry.value())
            .map(|entry| entry.key().clone())
            .collect();

        for key in expired_keys {
            self.remove_key(&key);
        }
    }

    // 删除键（从所有数据存储中）
    pub fn remove_key(&self, key: &str) {
        self.string_data.remove(key);
        self.hash_data.remove(key);
        self.list_data.remove(key);
        self.set_data.remove(key);
        self.sorted_set_data.remove(key);
        self.expiry_data.remove(key);
    }

    // 设置键的过期时间
    pub fn set_expiry(&self, key: &str, expiry_millis: u64) {
        self.expiry_data.insert(key.to_string(), expiry_millis);
    }

    // 移除键的过期时间
    pub fn remove_expiry(&self, key: &str) -> bool {
        self.expiry_data.remove(key).is_some()
    }

    // 获取键的剩余过期时间（毫秒），如果没有过期时间返回None
    pub fn get_ttl_millis(&self, key: &str) -> Option<i64> {
        if let Some(expiry) = self.expiry_data.get(key) {
            let now = self.current_timestamp_millis();
            let remaining = (*expiry as i64) - (now as i64);
            Some(remaining.max(-1)) // -1表示已过期
        } else {
            None // 没有设置过期时间
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}
