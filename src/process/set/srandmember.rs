use crate::{Data, Processor, Resp};
use crate::process::Parameter;

#[derive(Debug)]
pub struct SRandMemberCommandPara {
    key: String,
    count: Option<i64>,
    parameter: Parameter,
}

impl SRandMemberCommandPara {
    pub fn new(key: String, count: Option<i64>, parameter: Parameter) -> Self {
        Self { key, count, parameter }
    }
}

impl Processor for SRandMemberCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查键是否过期
        if data.is_expired(&self.key) {
            data.remove_key(&self.key);
            return Ok(Resp::Nulls(crate::resp::Nulls::new()));
        }

        if let Some(set) = data.set_data.get(&self.key) {
            let members: Vec<String> = set.iter().cloned().collect();
            
            if members.is_empty() {
                return Ok(Resp::Nulls(crate::resp::Nulls::new()));
            }
            
            if let Some(count) = self.count {
                let selected_count = std::cmp::min(count.abs() as usize, members.len());
                let selected: Vec<Resp> = members.iter()
                    .take(selected_count)
                    .map(|member| Resp::BulkStrings(crate::resp::BulkStrings::new(member.clone())))
                    .collect();
                
                Ok(Resp::Arrays(crate::resp::Arrays::new(selected)))
            } else {
                // 返回单个随机成员
                if let Some(member) = members.first() {
                    Ok(Resp::BulkStrings(crate::resp::BulkStrings::new(member.clone())))
                } else {
                    Ok(Resp::Nulls(crate::resp::Nulls::new()))
                }
            }
        } else {
            Ok(Resp::Nulls(crate::resp::Nulls::new()))
        }
    }
}