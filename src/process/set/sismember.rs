use crate::{Data, Processor, Resp};
use crate::process::Parameter;

#[derive(Debug)]
pub struct SIsMemberCommandPara {
    key: String,
    member: String,
    parameter: Parameter,
}

impl SIsMemberCommandPara {
    pub fn new(key: String, member: String, parameter: Parameter) -> Self {
        Self { key, member, parameter }
    }
}

impl Processor for SIsMemberCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查键是否过期
        if data.is_expired(&self.key) {
            data.remove_key(&self.key);
            return Ok(Resp::Integers(crate::resp::Integers::new(0)));
        }

        let is_member = data.set_data.get(&self.key)
            .map(|set| set.contains(&self.member))
            .unwrap_or(false);

        Ok(Resp::Integers(crate::resp::Integers::new(if is_member { 1 } else { 0 })))
    }
}