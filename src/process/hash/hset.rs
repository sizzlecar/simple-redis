use std::collections::HashMap;
use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp};

#[derive(Debug)]
pub struct HSetCommandPara {
    pub key: String,
    pub field_values: Vec<(String, String)>,
    para: Parameter,
}

impl HSetCommandPara {
    pub fn new(key: String, field_values: Vec<(String, String)>, para: Parameter) -> Self {
        Self { key, field_values, para }
    }
}

impl Processor for HSetCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("HSetCommandPara process start: {:?}", &self);
        
        let mut new_fields = 0i64;
        
        // 获取或创建hash
        let mut hash = data.hash_data.get(&self.key)
            .map(|entry| entry.value().clone())
            .unwrap_or_else(HashMap::new);
        
        for (field, value) in &self.field_values {
            let is_new = !hash.contains_key(field);
            hash.insert(field.clone(), value.clone());
            if is_new {
                new_fields += 1;
            }
        }
        
        data.hash_data.insert(self.key.clone(), hash);
        
        Ok(Resp::Integers(Integers::new(new_fields)))
    }
} 