use crate::process::Parameter;
use crate::{Data, Processor, Resp};

#[derive(Debug)]
pub struct ZRemRangeByRankCommandPara {
    key: String,
    start: i64,
    stop: i64,
    parameter: Parameter,
}

impl ZRemRangeByRankCommandPara {
    pub fn new(key: String, start: i64, stop: i64, parameter: Parameter) -> Self {
        Self {
            key,
            start,
            stop,
            parameter,
        }
    }
}

impl Processor for ZRemRangeByRankCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        Ok(Resp::Integers(crate::resp::Integers::new(0)))
    }
}
