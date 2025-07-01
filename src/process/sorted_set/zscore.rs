use crate::process::Parameter;
use crate::{Data, Processor, Resp};

#[derive(Debug)]
pub struct ZScoreCommandPara {
    key: String,
    member: String,
    parameter: Parameter,
}

impl ZScoreCommandPara {
    pub fn new(key: String, member: String, parameter: Parameter) -> Self {
        Self {
            key,
            member,
            parameter,
        }
    }
}

impl Processor for ZScoreCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查键是否过期
        if data.is_expired(&self.key) {
            data.remove_key(&self.key);
            return Ok(Resp::Nulls(crate::resp::Nulls::new()));
        }

        let score = data
            .sorted_set_data
            .get(&self.key)
            .and_then(|sorted_set| sorted_set.get(&self.member).copied());

        match score {
            Some(score) => Ok(Resp::BulkStrings(crate::resp::BulkStrings::new(
                score.to_string(),
            ))),
            None => Ok(Resp::Nulls(crate::resp::Nulls::new())),
        }
    }
}
