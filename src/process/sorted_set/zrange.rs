use crate::{Data, Processor, Resp};
use crate::process::Parameter;

#[derive(Debug)]
pub struct ZRangeCommandPara {
    key: String,
    start: i64,
    stop: i64,
    with_scores: bool,
    parameter: Parameter,
}

impl ZRangeCommandPara {
    pub fn new(key: String, start: i64, stop: i64, with_scores: bool, parameter: Parameter) -> Self {
        Self { key, start, stop, with_scores, parameter }
    }
}

impl Processor for ZRangeCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 基础实现，返回空数组
        Ok(Resp::Arrays(crate::resp::Arrays::new(vec![])))
    }
}