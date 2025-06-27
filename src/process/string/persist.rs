use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp};

#[derive(Debug)]
pub struct PersistCommandPara {
    pub key: String,
    para: Parameter,
}

impl PersistCommandPara {
    pub fn new(key: String, para: Parameter) -> Self {
        Self { key, para }
    }
}

impl Processor for PersistCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("PersistCommandPara process start: {:?}", &self);

        // 检查键是否存在
        let key_exists = data.string_data.contains_key(&self.key)
            || data.hash_data.contains_key(&self.key)
            || data.list_data.contains_key(&self.key)
            || data.set_data.contains_key(&self.key)
            || data.sorted_set_data.contains_key(&self.key);

        if key_exists {
            let removed = data.remove_expiry(&self.key);
            if removed {
                info!("⏰ PERSIST '{}' expiry removed", self.key);
                Ok(Resp::Integers(Integers::new(1))) // 成功移除过期时间
            } else {
                info!("⏰ PERSIST '{}' no expiry to remove", self.key);
                Ok(Resp::Integers(Integers::new(0))) // 没有过期时间需要移除
            }
        } else {
            info!("⏰ PERSIST '{}' key not found", self.key);
            Ok(Resp::Integers(Integers::new(0))) // 键不存在
        }
    }
}
