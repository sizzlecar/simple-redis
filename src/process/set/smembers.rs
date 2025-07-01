use crate::{Data, Processor, Resp};
use crate::process::Parameter;

#[derive(Debug)]
pub struct SMembersCommandPara {
    key: String,
    parameter: Parameter,
}

impl SMembersCommandPara {
    pub fn new(key: String, parameter: Parameter) -> Self {
        Self { key, parameter }
    }
}

impl Processor for SMembersCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查键是否过期
        if data.is_expired(&self.key) {
            data.remove_key(&self.key);
            return Ok(Resp::Arrays(crate::resp::Arrays::new(vec![])));
        }

        let members = data.set_data.get(&self.key)
            .map(|set| {
                set.iter()
                    .map(|member| Resp::BulkStrings(crate::resp::BulkStrings::new(member.clone())))
                    .collect()
            })
            .unwrap_or_else(Vec::new);

        Ok(Resp::Arrays(crate::resp::Arrays::new(members)))
    }
}