use tracing::info;

use crate::{process::Parameter, Data, Integers, Processor, Resp};

#[derive(Debug)]
pub struct DelCommandPara {
    pub keys: Vec<String>,
    #[allow(dead_code)]
    para: Parameter,
}

impl DelCommandPara {
    pub fn new(keys: Vec<String>, para: Parameter) -> Self {
        Self { keys, para }
    }
}

impl Processor for DelCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("DelCommandPara process start: {:?}", &self);
        let mut deleted_count = 0i64;

        for key in &self.keys {
            let removed = data.string_data.remove(key).is_some()
                || data.hash_data.remove(key).is_some()
                || data.list_data.remove(key).is_some()
                || data.set_data.remove(key).is_some()
                || data.sorted_set_data.remove(key).is_some();

            if removed {
                deleted_count += 1;
                // 同时移除过期时间
                data.expiry_data.remove(key);
            }
        }

        Ok(Resp::Integers(Integers::new(deleted_count)))
    }
}
