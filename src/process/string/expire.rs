use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp};

#[derive(Debug)]
pub struct ExpireCommandPara {
    pub key: String,
    pub seconds: u64,
    para: Parameter,
}

impl ExpireCommandPara {
    pub fn new(key: String, seconds: u64, para: Parameter) -> Self {
        Self { key, seconds, para }
    }
}

impl Processor for ExpireCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("ExpireCommandPara process start: {:?}", &self);

        // 检查键是否存在
        let key_exists = data.string_data.contains_key(&self.key)
            || data.hash_data.contains_key(&self.key)
            || data.list_data.contains_key(&self.key)
            || data.set_data.contains_key(&self.key)
            || data.sorted_set_data.contains_key(&self.key);

        if key_exists {
            // 计算过期时间戳（毫秒）
            let now = data.current_timestamp_millis();
            let expiry_millis = now + (self.seconds * 1000);

            data.set_expiry(&self.key, expiry_millis);
            info!(
                "⏰ EXPIRE '{}' set to expire in {} seconds",
                self.key, self.seconds
            );
            Ok(Resp::Integers(Integers::new(1)))
        } else {
            info!("⏰ EXPIRE '{}' key not found", self.key);
            Ok(Resp::Integers(Integers::new(0)))
        }
    }
}
