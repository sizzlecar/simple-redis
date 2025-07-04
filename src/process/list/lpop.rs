use tracing::info;

use crate::{process::Parameter, BulkStrings, Data, Nulls, Processor, Resp};

#[derive(Debug)]
pub struct LPopCommandPara {
    pub key: String,
    pub count: Option<i64>,
    #[allow(dead_code)]
    para: Parameter,
}

impl LPopCommandPara {
    pub fn new(key: String, count: Option<i64>, para: Parameter) -> Self {
        Self { key, count, para }
    }
}

impl Processor for LPopCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("LPopCommandPara process start: {:?}", &self);

        match data.list_data.get_mut(&self.key) {
            Some(mut list) => {
                if list.is_empty() {
                    return Ok(Resp::Nulls(Nulls::new()));
                }

                match self.count {
                    Some(count) if count > 1 => {
                        let mut results = Vec::new();
                        for _ in 0..count {
                            if let Some(value) = list.pop_front() {
                                results.push(Resp::BulkStrings(BulkStrings::new(value)));
                            } else {
                                break;
                            }
                        }

                        if list.is_empty() {
                            data.list_data.remove(&self.key);
                        }

                        Ok(Resp::Arrays(crate::Arrays::new(results)))
                    }
                    _ => {
                        // 单个元素弹出
                        match list.pop_front() {
                            Some(value) => {
                                if list.is_empty() {
                                    data.list_data.remove(&self.key);
                                }
                                Ok(Resp::BulkStrings(BulkStrings::new(value)))
                            }
                            None => Ok(Resp::Nulls(Nulls::new())),
                        }
                    }
                }
            }
            None => Ok(Resp::Nulls(Nulls::new())),
        }
    }
}
