use crate::{Data, Processor, Resp};
use crate::process::Parameter;

#[derive(Debug)]
pub struct ZRemRangeByScoreCommandPara {
    key: String,
    min: f64,
    max: f64,
    parameter: Parameter,
}

impl ZRemRangeByScoreCommandPara {
    pub fn new(key: String, min: f64, max: f64, parameter: Parameter) -> Self {
        Self { key, min, max, parameter }
    }
}

impl Processor for ZRemRangeByScoreCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        Ok(Resp::Integers(crate::resp::Integers::new(0)))
    }
}