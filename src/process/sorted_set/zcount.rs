use crate::process::Parameter;
use crate::{Data, Processor, Resp};

#[derive(Debug)]
pub struct ZCountCommandPara {
    key: String,
    min: f64,
    max: f64,
    parameter: Parameter,
}

impl ZCountCommandPara {
    pub fn new(key: String, min: f64, max: f64, parameter: Parameter) -> Self {
        Self {
            key,
            min,
            max,
            parameter,
        }
    }
}

impl Processor for ZCountCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        Ok(Resp::Integers(crate::resp::Integers::new(0)))
    }
}
