use crate::{Data, Processor, Resp};
use crate::process::Parameter;
use std::collections::HashSet;

#[derive(Debug)]
pub struct SMoveCommandPara {
    source: String,
    destination: String,
    member: String,
    parameter: Parameter,
}

impl SMoveCommandPara {
    pub fn new(source: String, destination: String, member: String, parameter: Parameter) -> Self {
        Self {
            source,
            destination,
            member,
            parameter,
        }
    }
}

impl Processor for SMoveCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查源键是否过期
        if data.is_expired(&self.source) {
            data.remove_key(&self.source);
            return Ok(Resp::Integers(crate::resp::Integers::new(0)));
        }

        // 检查目标键是否过期
        if data.is_expired(&self.destination) {
            data.remove_key(&self.destination);
        }

        let moved = if let Some(mut source_set) = data.set_data.get_mut(&self.source) {
            if source_set.remove(&self.member) {
                // 如果源集合为空，删除键
                if source_set.is_empty() {
                    drop(source_set); // 释放可变引用
                    data.set_data.remove(&self.source);
                }
                
                // 添加到目标集合
                let mut dest_set = data.set_data.entry(self.destination.clone()).or_insert_with(HashSet::new);
                dest_set.insert(self.member.clone());
                true
            } else {
                false
            }
        } else {
            false
        };

        Ok(Resp::Integers(crate::resp::Integers::new(if moved { 1 } else { 0 })))
    }
}