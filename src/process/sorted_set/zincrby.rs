use crate::process::Parameter;
use crate::{Data, Processor, Resp};

#[derive(Debug)]
pub struct ZIncrByCommandPara {
    key: String,
    increment: f64,
    member: String,
    parameter: Parameter,
}

impl ZIncrByCommandPara {
    pub fn new(key: String, increment: f64, member: String, parameter: Parameter) -> Self {
        Self {
            key,
            increment,
            member,
            parameter,
        }
    }
}

impl Processor for ZIncrByCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        Ok(Resp::BulkStrings(crate::resp::BulkStrings::new(
            "0".to_string(),
        )))
    }
}
