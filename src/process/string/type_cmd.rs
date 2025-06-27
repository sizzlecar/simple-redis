use tracing::info;

use crate::{process::Parameter, Data, Processor, Resp, SimpleStringsData};

#[derive(Debug)]
pub struct TypeCommandPara {
    pub key: String,
    #[allow(dead_code)]
    para: Parameter,
}

impl TypeCommandPara {
    pub fn new(key: String, para: Parameter) -> Self {
        Self { key, para }
    }
}

impl Processor for TypeCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查各种数据类型中是否存在该键
        let key_type = if data.string_data.contains_key(&self.key) {
            "string"
        } else if data.hash_data.contains_key(&self.key) {
            "hash"
        } else if data.list_data.contains_key(&self.key) {
            "list"
        } else if data.set_data.contains_key(&self.key) {
            "set"
        } else if data.sorted_set_data.contains_key(&self.key) {
            "zset"
        } else {
            "none"
        };

        info!("🔍 TYPE '{}' -> {}", self.key, key_type);
        Ok(Resp::SimpleStrings(SimpleStringsData::new(
            key_type.to_string(),
        )))
    }
}
