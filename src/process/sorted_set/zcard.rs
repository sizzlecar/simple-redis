use crate::process::Parameter;
use crate::{Data, Processor, Resp};

#[derive(Debug)]
pub struct ZCardCommandPara {
    key: String,
    parameter: Parameter,
}

impl ZCardCommandPara {
    pub fn new(key: String, parameter: Parameter) -> Self {
        Self { key, parameter }
    }
}

impl Processor for ZCardCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查键是否过期
        if data.is_expired(&self.key) {
            data.remove_key(&self.key);
            return Ok(Resp::Integers(crate::resp::Integers::new(0)));
        }

        let count = data
            .sorted_set_data
            .get(&self.key)
            .map(|sorted_set| sorted_set.len() as i64)
            .unwrap_or(0);

        Ok(Resp::Integers(crate::resp::Integers::new(count)))
    }
}
