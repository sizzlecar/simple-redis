use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp};

#[derive(Debug)]
pub struct ExistsCommandPara {
    pub keys: Vec<String>,
    para: Parameter,
}

impl ExistsCommandPara {
    pub fn new(keys: Vec<String>, para: Parameter) -> Self {
        Self { keys, para }
    }
}

impl Processor for ExistsCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("ExistsCommandPara process start: {:?}", &self);
        let mut exist_count = 0i64;
        
        for key in &self.keys {
            if data.string_data.contains_key(key)
                || data.hash_data.contains_key(key)
                || data.list_data.contains_key(key)
                || data.set_data.contains_key(key)
                || data.sorted_set_data.contains_key(key)
            {
                exist_count += 1;
            }
        }
        
        Ok(Resp::Integers(Integers::new(exist_count)))
    }
} 