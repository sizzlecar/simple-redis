use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp, SimpleErrors, SimpleStringsData};

#[derive(Debug)]
pub struct IncrCommandPara {
    pub key: String,
    pub increment: Option<i64>,
    #[allow(dead_code)]
    para: Parameter,
}

impl IncrCommandPara {
    pub fn new(key: String, increment: Option<i64>, para: Parameter) -> Self {
        Self {
            key,
            increment,
            para,
        }
    }
}

impl Processor for IncrCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("IncrCommandPara process start: {:?}", &self);

        let increment = self.increment.unwrap_or(1);

        match data.string_data.get(&self.key) {
            Some(entry) => {
                let current_value = match entry.value() {
                    Resp::SimpleStrings(s) => &s.val,
                    Resp::BulkStrings(s) => &s.val,
                    _ => {
                        return Ok(Resp::SimpleErrors(SimpleErrors::new(
                            "ERR value is not an integer or out of range".to_string(),
                        )))
                    }
                };

                match current_value.parse::<i64>() {
                    Ok(num) => {
                        let new_value = num + increment;
                        data.string_data.insert(
                            self.key.clone(),
                            Resp::SimpleStrings(SimpleStringsData::new(new_value.to_string())),
                        );
                        Ok(Resp::Integers(Integers::new(new_value)))
                    }
                    Err(_) => Ok(Resp::SimpleErrors(SimpleErrors::new(
                        "ERR value is not an integer or out of range".to_string(),
                    ))),
                }
            }
            None => {
                // 键不存在，初始化为0再增加
                data.string_data.insert(
                    self.key.clone(),
                    Resp::SimpleStrings(SimpleStringsData::new(increment.to_string())),
                );
                Ok(Resp::Integers(Integers::new(increment)))
            }
        }
    }
}
