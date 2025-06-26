use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp};

#[derive(Debug)]
pub struct DelCommandPara {
    pub keys: Vec<String>,
    para: Parameter,
}

impl DelCommandPara {
    pub fn new(keys: Vec<String>, para: Parameter) -> Self {
        Self { keys, para }
    }
}

impl Processor for DelCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("DelCommandPara process start: {:?}", &self);
        let mut deleted_count = 0i64;
        
        for key in &self.keys {
            if data.string_data.remove(key).is_some() {
                deleted_count += 1;
            } else if data.hash_data.remove(key).is_some() {
                deleted_count += 1;
            } else if data.list_data.remove(key).is_some() {
                deleted_count += 1;
            } else if data.set_data.remove(key).is_some() {
                deleted_count += 1;
            } else if data.sorted_set_data.remove(key).is_some() {
                deleted_count += 1;
            }
        }
        
        Ok(Resp::Integers(Integers::new(deleted_count)))
    }
} 