use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp};

#[derive(Debug)]
pub struct HSetCommandPara {
    pub key: String,
    pub field_values: Vec<(String, String)>,
    #[allow(dead_code)]
    para: Parameter,
}

impl HSetCommandPara {
    pub fn new(key: String, field_values: Vec<(String, String)>, para: Parameter) -> Self {
        Self {
            key,
            field_values,
            para,
        }
    }
}

impl Processor for HSetCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        let mut hash_map = data
            .hash_data
            .get(&self.key)
            .map(|entry| entry.value().clone())
            .unwrap_or_default();

        let mut new_fields = 0;
        for (field, value) in &self.field_values {
            if !hash_map.contains_key(field) {
                new_fields += 1;
            }
            hash_map.insert(field.clone(), value.clone());
        }

        data.hash_data.insert(self.key.clone(), hash_map);

        info!("✅ HSET '{}' -> {} new fields added", self.key, new_fields);
        Ok(Resp::Integers(Integers::new(new_fields as i64)))
    }
}
