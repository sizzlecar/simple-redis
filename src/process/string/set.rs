use anyhow::Ok;
use tracing::info;

use crate::{process::Parameter, Data, Processor, Resp, SimpleStringsData};

#[derive(Debug)]
#[allow(unused)]
pub struct SetCommandPara {
    pub key: Option<String>,

    pub value: Option<String>,

    para: Parameter,
}

impl SetCommandPara {
    pub fn new(key: Option<String>, value: Option<String>, para: Parameter) -> Self {
        Self { key, value, para }
    }
}

impl Processor for SetCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!(
            "SetCommandPara process start: {:?}, data: {:?}",
            &self, data
        );
        match &self.key {
            Some(k) => {
                let val = self
                    .value
                    .clone()
                    .ok_or_else(|| anyhow::Error::msg("value is none"))?;
                data.string_data
                    .insert(k.clone(), Resp::SimpleStrings(SimpleStringsData::new(val)));
                Ok(Resp::SimpleStrings(SimpleStringsData::new("OK".to_owned())))
            }
            None => Err(anyhow::Error::msg("key is none")),
        }
    }
}
