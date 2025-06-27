use tracing::info;

use crate::{process::Parameter, Arrays, BulkStrings, Data, Processor, Resp};

#[derive(Debug)]
pub struct HGetAllCommandPara {
    pub key: String,
    #[allow(dead_code)]
    para: Parameter,
}

impl HGetAllCommandPara {
    pub fn new(key: String, para: Parameter) -> Self {
        Self { key, para }
    }
}

impl Processor for HGetAllCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("HGetAllCommandPara process start: {:?}", &self);

        match data.hash_data.get(&self.key) {
            Some(hash) => {
                let mut result = Vec::new();
                for (field, value) in hash.iter() {
                    result.push(Resp::BulkStrings(BulkStrings::new(field.clone())));
                    result.push(Resp::BulkStrings(BulkStrings::new(value.clone())));
                }
                Ok(Resp::Arrays(Arrays::new(result)))
            }
            None => Ok(Resp::Arrays(Arrays::new(Vec::new()))),
        }
    }
}
