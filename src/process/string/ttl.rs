use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp};

#[derive(Debug)]
pub struct TtlCommandPara {
    pub key: String,
    #[allow(dead_code)]
    para: Parameter,
}

impl TtlCommandPara {
    pub fn new(key: String, para: Parameter) -> Self {
        Self { key, para }
    }
}

impl Processor for TtlCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("TtlCommandPara process start: {:?}", &self);

        // 检查键是否存在
        let key_exists = data.string_data.contains_key(&self.key)
            || data.hash_data.contains_key(&self.key)
            || data.list_data.contains_key(&self.key)
            || data.set_data.contains_key(&self.key)
            || data.sorted_set_data.contains_key(&self.key);

        if !key_exists {
            info!("⏰ TTL '{}' key not found", self.key);
            return Ok(Resp::Integers(Integers::new(-2))); // 键不存在
        }

        match data.get_ttl_millis(&self.key) {
            Some(ttl_millis) => {
                if ttl_millis < 0 {
                    // 键已过期，清理它
                    data.remove_key(&self.key);
                    info!("⏰ TTL '{}' key expired and removed", self.key);
                    Ok(Resp::Integers(Integers::new(-2))) // 键不存在
                } else {
                    let ttl_seconds = ttl_millis / 1000;
                    info!("⏰ TTL '{}' -> {} seconds", self.key, ttl_seconds);
                    Ok(Resp::Integers(Integers::new(ttl_seconds)))
                }
            }
            None => {
                info!("⏰ TTL '{}' no expiry set", self.key);
                Ok(Resp::Integers(Integers::new(-1))) // 没有设置过期时间
            }
        }
    }
}
