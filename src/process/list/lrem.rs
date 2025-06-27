use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp};

#[derive(Debug)]
pub struct LRemCommandPara {
    pub key: String,
    pub count: i64,
    pub element: String,
    #[allow(dead_code)]
    para: Parameter,
}

impl LRemCommandPara {
    pub fn new(key: String, count: i64, element: String, para: Parameter) -> Self {
        Self {
            key,
            count,
            element,
            para,
        }
    }
}

impl Processor for LRemCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("LRemCommandPara process start: {:?}", &self);

        match data.list_data.get_mut(&self.key) {
            Some(mut list) => {
                let mut removed_count = 0;

                if self.count == 0 {
                    // 删除所有匹配的元素
                    list.retain(|item| {
                        if item == &self.element {
                            removed_count += 1;
                            false
                        } else {
                            true
                        }
                    });
                } else if self.count > 0 {
                    // 从头开始删除指定数量的匹配元素
                    let mut remaining = self.count;
                    let mut i = 0;
                    while i < list.len() && remaining > 0 {
                        if list[i] == self.element {
                            list.remove(i);
                            removed_count += 1;
                            remaining -= 1;
                        } else {
                            i += 1;
                        }
                    }
                } else {
                    // 从尾开始删除指定数量的匹配元素
                    let mut remaining = (-self.count) as usize;
                    let mut i = list.len();
                    while i > 0 && remaining > 0 {
                        i -= 1;
                        if list[i] == self.element {
                            list.remove(i);
                            removed_count += 1;
                            remaining -= 1;
                        }
                    }
                }

                info!("🗑️ LREM '{}' removed {} elements", self.key, removed_count);
                Ok(Resp::Integers(Integers::new(removed_count)))
            }
            None => {
                info!("🗑️ LREM '{}' key not found", self.key);
                Ok(Resp::Integers(Integers::new(0)))
            }
        }
    }
}
