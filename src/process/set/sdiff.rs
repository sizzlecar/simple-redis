use crate::{Data, Processor, Resp};
use crate::process::Parameter;
use std::collections::HashSet;

#[derive(Debug)]
pub struct SDiffCommandPara {
    keys: Vec<String>,
    parameter: Parameter,
}

impl SDiffCommandPara {
    pub fn new(keys: Vec<String>, parameter: Parameter) -> Self {
        Self { keys, parameter }
    }
}

impl Processor for SDiffCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        if self.keys.is_empty() {
            return Ok(Resp::Arrays(crate::resp::Arrays::new(vec![])));
        }

        let first_key = &self.keys[0];
        
        // 检查第一个键是否过期
        if data.is_expired(first_key) {
            data.remove_key(first_key);
            return Ok(Resp::Arrays(crate::resp::Arrays::new(vec![])));
        }

        let mut result = data.set_data.get(first_key)
            .map(|set| set.clone())
            .unwrap_or_default();

        // 从第二个键开始，从结果中移除这些集合的成员
        for key in &self.keys[1..] {
            // 检查键是否过期
            if data.is_expired(key) {
                data.remove_key(key);
                continue;
            }

            if let Some(set) = data.set_data.get(key) {
                for member in set.iter() {
                    result.remove(member);
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