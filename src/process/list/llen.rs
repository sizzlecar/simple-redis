use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp};

#[derive(Debug)]
pub struct LLenCommandPara {
    pub key: String,
    para: Parameter,
}

impl LLenCommandPara {
    pub fn new(key: String, para: Parameter) -> Self {
        Self { key, para }
    }
}

impl Processor for LLenCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("LLenCommandPara process start: {:?}", &self);
        
        match data.list_data.get(&self.key) {
            Some(list) => Ok(Resp::Integers(Integers::new(list.len() as i64))),
            None => Ok(Resp::Integers(Integers::new(0))),
        }
    }
} 