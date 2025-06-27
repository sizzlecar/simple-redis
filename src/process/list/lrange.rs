use tracing::info;

use crate::{process::Parameter, Arrays, BulkStrings, Data, Processor, Resp};

#[derive(Debug)]
pub struct LRangeCommandPara {
    pub key: String,
    pub start: i64,
    pub stop: i64,
    #[allow(dead_code)]
    para: Parameter,
}

impl LRangeCommandPara {
    pub fn new(key: String, start: i64, stop: i64, para: Parameter) -> Self {
        Self {
            key,
            start,
            stop,
            para,
        }
    }
}

impl Processor for LRangeCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("LRangeCommandPara process start: {:?}", &self);

        match data.list_data.get(&self.key) {
            Some(list) => {
                let len = list.len() as i64;
                if len == 0 {
                    return Ok(Resp::Arrays(Arrays::new(Vec::new())));
                }

                // 处理负数索引
                let start = if self.start < 0 {
                    (len + self.start).max(0)
                } else {
                    self.start
                };

                let stop = if self.stop < 0 {
                    (len + self.stop).max(-1)
                } else {
                    self.stop
                };

                // 检查范围是否有效
                if start >= len || stop < 0 || start > stop {
                    return Ok(Resp::Arrays(Arrays::new(Vec::new())));
                }

                // 确保索引在有效范围内
                let start = start.max(0).min(len - 1);
                let stop = stop.max(0).min(len - 1);

                let mut result = Vec::new();
                let list_vec: Vec<_> = list.iter().collect();

                for i in start..=stop {
                    if i >= 0 && (i as usize) < list_vec.len() {
                        result.push(Resp::BulkStrings(BulkStrings::new(
                            list_vec[i as usize].clone(),
                        )));
                    }
                }

                Ok(Resp::Arrays(Arrays::new(result)))
            }
            None => Ok(Resp::Arrays(Arrays::new(Vec::new()))),
        }
    }
}
