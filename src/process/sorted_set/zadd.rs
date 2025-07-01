use crate::process::Parameter;
use crate::{Data, Processor, Resp};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct ZAddCommandPara {
    key: String,
    score_members: Vec<(f64, String)>,
    parameter: Parameter,
}

impl ZAddCommandPara {
    pub fn new(key: String, score_members: Vec<(f64, String)>, parameter: Parameter) -> Self {
        Self {
            key,
            score_members,
            parameter,
        }
    }
}

impl Processor for ZAddCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查键是否过期
        if data.is_expired(&self.key) {
            data.remove_key(&self.key);
        }

        let mut sorted_set = data
            .sorted_set_data
            .entry(self.key.clone())
            .or_insert_with(BTreeMap::new);
        let mut added_count = 0;

        for (score, member) in &self.score_members {
            if !sorted_set.contains_key(member) {
                added_count += 1;
            }
            sorted_set.insert(member.clone(), *score);
        }

        Ok(Resp::Integers(crate::resp::Integers::new(added_count)))
    }
}
