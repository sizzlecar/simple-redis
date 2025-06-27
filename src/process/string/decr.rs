use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp, SimpleErrors, SimpleStringsData};

#[derive(Debug)]
pub struct DecrCommandPara {
    pub key: String,
    pub decrement: Option<i64>,
    #[allow(dead_code)]
    para: Parameter,
}

impl DecrCommandPara {
    pub fn new(key: String, decrement: Option<i64>, para: Parameter) -> Self {
        Self {
            key,
            decrement,
            para,
        }
    }
}

impl Processor for DecrCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("DecrCommandPara process start: {:?}", &self);

        let decrement = self.decrement.unwrap_or(1);

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
                        let new_value = num - decrement;
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
                // 键不存在，初始化为0再减少
                let new_value = -decrement;
                data.string_data.insert(
                    self.key.clone(),
                    Resp::SimpleStrings(SimpleStringsData::new(new_value.to_string())),
                );
                Ok(Resp::Integers(Integers::new(new_value)))
            }
        }
    }
}
