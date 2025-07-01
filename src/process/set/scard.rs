use crate::{Data, Processor, Resp};
use crate::process::Parameter;

#[derive(Debug)]
pub struct SCardCommandPara {
    key: String,
    parameter: Parameter,
}

impl SCardCommandPara {
    pub fn new(key: String, parameter: Parameter) -> Self {
        Self { key, parameter }
    }
}

impl Processor for SCardCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查键是否过期
        if data.is_expired(&self.key) {
            data.remove_key(&self.key);
            return Ok(Resp::Integers(crate::resp::Integers::new(0)));
        }

        let count = data.set_data.get(&self.key)
            .map(|set| set.len() as i64)
            .unwrap_or(0);

        Ok(Resp::Integers(crate::resp::Integers::new(count)))
    }
}