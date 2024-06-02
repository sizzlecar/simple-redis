use tracing::info;

use crate::{process::Parameter, Data, Nulls, Processor, Resp, SimpleErrors};

#[derive(Debug)]
#[allow(unused)]
pub struct GetCommandPara {
    pub key: Option<String>,

    pub value: Option<String>,

    para: Parameter,
}

impl GetCommandPara {
    pub fn new(key: Option<String>, value: Option<String>, para: Parameter) -> Self {
        Self { key, value, para }
    }
}

impl Processor for GetCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!(
            "GetCommandPara process start: {:?}, data: {:?}",
            &self, data
        );
        match &self.key {
            Some(k) => {
                if !data.string_data.contains_key(k) {
                    Ok(Resp::SimpleErrors(SimpleErrors::new(
                        "no suck key".to_owned(),
                    )))
                } else {
                    let v = data.string_data.get(k);
                    match v {
                        Some(v) => Ok(v.value().clone()),
                        None => Ok(Resp::Nulls(Nulls::default())),
                    }
                }
            }
            None => Err(anyhow::Error::msg("key is none")),
        }
    }
}
