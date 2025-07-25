use crate::process::Parameter;
use crate::{Data, Processor, Resp};

#[derive(Debug)]
pub struct ZRevRankCommandPara {
    key: String,
    member: String,
    parameter: Parameter,
}

impl ZRevRankCommandPara {
    pub fn new(key: String, member: String, parameter: Parameter) -> Self {
        Self {
            key,
            member,
            parameter,
        }
    }
}

impl Processor for ZRevRankCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        Ok(Resp::Nulls(crate::resp::Nulls::new()))
    }
}
