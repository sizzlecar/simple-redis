use crate::process::Parameter;
use crate::{Data, Processor, Resp};
use std::collections::HashSet;

#[derive(Debug)]
pub struct SUnionCommandPara {
    keys: Vec<String>,
    parameter: Parameter,
}

impl SUnionCommandPara {
    pub fn new(keys: Vec<String>, parameter: Parameter) -> Self {
        Self { keys, parameter }
    }
}

impl Processor for SUnionCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        let mut result = HashSet::new();

        for key in &self.keys {
            // 检查键是否过期
            if data.is_expired(key) {
                data.remove_key(key);
                continue;
            }

            if let Some(set) = data.set_data.get(key) {
                for member in set.iter() {
                    result.insert(member.clone());
                }
            }
        }

        let members = result
            .into_iter()
            .map(|member| Resp::BulkStrings(crate::resp::BulkStrings::new(member)))
            .collect();

        Ok(Resp::Arrays(crate::resp::Arrays::new(members)))
    }
}
