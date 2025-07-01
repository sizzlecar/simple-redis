use crate::{Data, Processor, Resp};
use crate::process::Parameter;

#[derive(Debug)]
pub struct SRemCommandPara {
    key: String,
    members: Vec<String>,
    parameter: Parameter,
}

impl SRemCommandPara {
    pub fn new(key: String, members: Vec<String>, parameter: Parameter) -> Self {
        Self {
            key,
            members,
            parameter,
        }
    }
}

impl Processor for SRemCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查键是否过期
        if data.is_expired(&self.key) {
            data.remove_key(&self.key);
            return Ok(Resp::Integers(crate::resp::Integers::new(0)));
        }

        let mut removed_count = 0;
        
        if let Some(mut set) = data.set_data.get_mut(&self.key) {
            for member in &self.members {
                if set.remove(member) {
                    removed_count += 1;
                }
            }
            
            // 如果集合为空，删除键
            if set.is_empty() {
                drop(set); // 释放可变引用
                data.set_data.remove(&self.key);
            }
        }

        Ok(Resp::Integers(crate::resp::Integers::new(removed_count)))
    }
}