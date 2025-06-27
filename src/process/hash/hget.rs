use tracing::info;

use crate::{process::Parameter, BulkStrings, Data, Nulls, Processor, Resp};

#[derive(Debug)]
pub struct HGetCommandPara {
    pub key: String,
    pub field: String,
    para: Parameter,
}

impl HGetCommandPara {
    pub fn new(key: String, field: String, para: Parameter) -> Self {
        Self { key, field, para }
    }
}

impl Processor for HGetCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        match data.hash_data.get(&self.key) {
            Some(hash_entry) => {
                let hash = hash_entry.value();
                match hash.get(&self.field) {
                    Some(value) => {
                        info!(
                            "✅ HGET '{}' '{}' -> found: '{}'",
                            self.key, self.field, value
                        );
                        Ok(Resp::BulkStrings(BulkStrings::new(value.clone())))
                    }
                    None => {
                        info!(
                            "❌ HGET '{}' '{}' -> field not found (NULL)",
                            self.key, self.field
                        );
                        Ok(Resp::Nulls(Nulls::new()))
                    }
                }
            }
            None => {
                info!(
                    "❌ HGET '{}' '{}' -> key not found (NULL)",
                    self.key, self.field
                );
                Ok(Resp::Nulls(Nulls::new()))
            }
        }
    }
}
