use crate::process::Parameter;
use crate::{Data, Processor, Resp};
use std::collections::HashSet;

#[derive(Debug)]
pub struct SPopCommandPara {
    key: String,
    count: Option<i64>,
    parameter: Parameter,
}

impl SPopCommandPara {
    pub fn new(key: String, count: Option<i64>, parameter: Parameter) -> Self {
        Self {
            key,
            count,
            parameter,
        }
    }
}

impl Processor for SPopCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查键是否过期
        if data.is_expired(&self.key) {
            data.remove_key(&self.key);
            return Ok(Resp::Nulls(crate::resp::Nulls::new()));
        }

        let count = self.count.unwrap_or(1);

        if let Some(mut set) = data.set_data.get_mut(&self.key) {
            let mut popped = Vec::new();
            let members: Vec<String> = set.iter().cloned().collect();

            let pop_count = std::cmp::min(count as usize, members.len());

            for i in 0..pop_count {
                if let Some(member) = members.get(i) {
                    if set.remove(member) {
                        popped.push(Resp::BulkStrings(crate::resp::BulkStrings::new(
                            member.clone(),
                        )));
                    }
                }
            }

            // 如果集合为空，删除键
            if set.is_empty() {
                drop(set); // 释放可变引用
                data.set_data.remove(&self.key);
            }

            if self.count.is_some() {
                Ok(Resp::Arrays(crate::resp::Arrays::new(popped)))
            } else {
                Ok(popped
                    .into_iter()
                    .next()
                    .unwrap_or(Resp::Nulls(crate::resp::Nulls::new())))
            }
        } else {
            Ok(Resp::Nulls(crate::resp::Nulls::new()))
        }
    }
}
