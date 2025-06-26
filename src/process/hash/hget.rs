use tracing::info;

use crate::{process::Parameter, Data, BulkStrings, Nulls, Processor, Resp};

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
        info!("HGetCommandPara process start: {:?}", &self);
        
        match data.hash_data.get(&self.key) {
            Some(hash) => {
                match hash.get(&self.field) {
                    Some(value) => Ok(Resp::BulkStrings(BulkStrings::new(value.clone()))),
                    None => Ok(Resp::Nulls(Nulls::new())),
                }
            }
            None => Ok(Resp::Nulls(Nulls::new())),
        }
    }
} 