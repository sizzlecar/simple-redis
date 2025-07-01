use crate::process::Parameter;
use crate::{Data, Processor, Resp};

#[derive(Debug)]
pub struct ZRemCommandPara {
    key: String,
    members: Vec<String>,
    parameter: Parameter,
}

impl ZRemCommandPara {
    pub fn new(key: String, members: Vec<String>, parameter: Parameter) -> Self {
        Self {
            key,
            members,
            parameter,
        }
    }
}

impl Processor for ZRemCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查键是否过期
        if data.is_expired(&self.key) {
            data.remove_key(&self.key);
            return Ok(Resp::Integers(crate::resp::Integers::new(0)));
        }

        let mut removed_count = 0;

        if let Some(mut sorted_set) = data.sorted_set_data.get_mut(&self.key) {
            for member in &self.members {
                if sorted_set.remove(member).is_some() {
                    removed_count += 1;
                }
            }

            // 如果有序集合为空，删除键
            if sorted_set.is_empty() {
                drop(sorted_set); // 释放可变引用
                data.sorted_set_data.remove(&self.key);
            }
        }

        Ok(Resp::Integers(crate::resp::Integers::new(removed_count)))
    }
}
