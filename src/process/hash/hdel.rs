use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp};

#[derive(Debug)]
pub struct HDelCommandPara {
    pub key: String,
    pub fields: Vec<String>,
    para: Parameter,
}

impl HDelCommandPara {
    pub fn new(key: String, fields: Vec<String>, para: Parameter) -> Self {
        Self { key, fields, para }
    }
}

impl Processor for HDelCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("HDelCommandPara process start: {:?}", &self);

        let mut deleted_count = 0i64;

        if let Some(mut hash_entry) = data.hash_data.get_mut(&self.key) {
            for field in &self.fields {
                if hash_entry.remove(field).is_some() {
                    deleted_count += 1;
                }
            }

            // 如果hash为空，删除整个键
            if hash_entry.is_empty() {
                data.hash_data.remove(&self.key);
            }
        }

        Ok(Resp::Integers(Integers::new(deleted_count)))
    }
}
