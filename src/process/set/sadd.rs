use crate::{Data, Processor, Resp};
use crate::process::Parameter;
use std::collections::HashSet;

#[derive(Debug)]
pub struct SAddCommandPara {
    key: String,
    members: Vec<String>,
    parameter: Parameter,
}

impl SAddCommandPara {
    pub fn new(key: String, members: Vec<String>, parameter: Parameter) -> Self {
        Self {
            key,
            members,
            parameter,
        }
    }
}

impl Processor for SAddCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查键是否过期
        if data.is_expired(&self.key) {
            data.remove_key(&self.key);
        }

        let mut set = data.set_data.entry(self.key.clone()).or_insert_with(HashSet::new);
        let mut added_count = 0;

        for member in &self.members {
            if set.insert(member.clone()) {
                added_count += 1;
            }
        }

        Ok(Resp::Integers(crate::resp::Integers::new(added_count)))
    }
}