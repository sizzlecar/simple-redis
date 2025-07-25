use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp};

#[derive(Debug)]
pub struct RPushCommandPara {
    pub key: String,
    pub values: Vec<String>,
    #[allow(dead_code)]
    para: Parameter,
}

impl RPushCommandPara {
    pub fn new(key: String, values: Vec<String>, para: Parameter) -> Self {
        Self { key, values, para }
    }
}

impl Processor for RPushCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("RPushCommandPara process start: {:?}", &self);

        let mut list = data
            .list_data
            .get(&self.key)
            .map(|entry| entry.value().clone())
            .unwrap_or_default();

        for value in &self.values {
            list.push_back(value.clone());
        }

        let new_length = list.len() as i64;
        data.list_data.insert(self.key.clone(), list);

        Ok(Resp::Integers(Integers::new(new_length)))
    }
}
