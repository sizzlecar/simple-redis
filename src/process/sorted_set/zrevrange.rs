use crate::process::Parameter;
use crate::{Data, Processor, Resp};

#[derive(Debug)]
pub struct ZRevRangeCommandPara {
    key: String,
    start: i64,
    stop: i64,
    with_scores: bool,
    parameter: Parameter,
}

impl ZRevRangeCommandPara {
    pub fn new(
        key: String,
        start: i64,
        stop: i64,
        with_scores: bool,
        parameter: Parameter,
    ) -> Self {
        Self {
            key,
            start,
            stop,
            with_scores,
            parameter,
        }
    }
}

impl Processor for ZRevRangeCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        Ok(Resp::Arrays(crate::resp::Arrays::new(vec![])))
    }
}
