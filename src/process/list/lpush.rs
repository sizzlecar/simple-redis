use std::collections::VecDeque;
use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp};

#[derive(Debug)]
pub struct LPushCommandPara {
    pub key: String,
    pub values: Vec<String>,
    para: Parameter,
}

impl LPushCommandPara {
    pub fn new(key: String, values: Vec<String>, para: Parameter) -> Self {
        Self { key, values, para }
    }
}

impl Processor for LPushCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("LPushCommandPara process start: {:?}", &self);

        let mut list = data
            .list_data
            .get(&self.key)
            .map(|entry| entry.value().clone())
            .unwrap_or_else(VecDeque::new);

        for value in self.values.iter().rev() {
            list.push_front(value.clone());
        }

        let new_length = list.len() as i64;
        data.list_data.insert(self.key.clone(), list);

        Ok(Resp::Integers(Integers::new(new_length)))
    }
}
