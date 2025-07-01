use crate::process::Parameter;
use crate::{Data, Processor, Resp};
use std::collections::HashSet;

#[derive(Debug)]
pub struct SInterCommandPara {
    keys: Vec<String>,
    parameter: Parameter,
}

impl SInterCommandPara {
    pub fn new(keys: Vec<String>, parameter: Parameter) -> Self {
        Self { keys, parameter }
    }
}

impl Processor for SInterCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        if self.keys.is_empty() {
            return Ok(Resp::Arrays(crate::resp::Arrays::new(vec![])));
        }

        let mut result: Option<HashSet<String>> = None;

        for key in &self.keys {
            // 检查键是否过期
            if data.is_expired(key) {
                data.remove_key(key);
                return Ok(Resp::Arrays(crate::resp::Arrays::new(vec![])));
            }

            if let Some(set) = data.set_data.get(key) {
                match result {
                    None => {
                        result = Some(set.clone());
                    }
                    Some(ref mut current) => {
                        current.retain(|member| set.contains(member));
                    }
                }
            } else {
                // 如果任何一个键不存在，交集为空
                return Ok(Resp::Arrays(crate::resp::Arrays::new(vec![])));
            }
        }

        let members = result
            .unwrap_or_default()
            .into_iter()
            .map(|member| Resp::BulkStrings(crate::resp::BulkStrings::new(member)))
            .collect();

        Ok(Resp::Arrays(crate::resp::Arrays::new(members)))
    }
}
