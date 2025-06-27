use tracing::info;

use crate::{process::Parameter, Arrays, BulkStrings, Data, Processor, Resp};

#[derive(Debug)]
pub struct KeysCommandPara {
    pub pattern: String,
    #[allow(dead_code)]
    para: Parameter,
}

impl KeysCommandPara {
    pub fn new(pattern: String, para: Parameter) -> Self {
        Self { pattern, para }
    }
}

impl Processor for KeysCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        let mut keys = Vec::new();

        // 收集所有类型的键
        for key in data.string_data.iter() {
            keys.push(key.key().clone());
        }

        for key in data.hash_data.iter() {
            keys.push(key.key().clone());
        }

        for key in data.list_data.iter() {
            keys.push(key.key().clone());
        }

        for key in data.set_data.iter() {
            keys.push(key.key().clone());
        }

        for key in data.sorted_set_data.iter() {
            keys.push(key.key().clone());
        }

        // 简单的模式匹配（只支持 * 通配符）
        let filtered_keys: Vec<String> = if self.pattern == "*" {
            keys
        } else {
            keys.into_iter()
                .filter(|key| key.contains(&self.pattern.replace("*", "")))
                .collect()
        };

        info!(
            "🔑 KEYS '{}' -> {} keys found",
            self.pattern,
            filtered_keys.len()
        );

        let resp_keys: Vec<Resp> = filtered_keys
            .into_iter()
            .map(|key| Resp::BulkStrings(BulkStrings::new(key)))
            .collect();

        Ok(Resp::Arrays(Arrays::new(resp_keys)))
    }
}
