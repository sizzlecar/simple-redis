use tracing::info;

use crate::{process::Parameter, Arrays, BulkStrings, Data, Processor, Resp};

#[derive(Debug)]
pub struct HValsCommandPara {
    pub key: String,
    para: Parameter,
}

impl HValsCommandPara {
    pub fn new(key: String, para: Parameter) -> Self {
        Self { key, para }
    }
}

impl Processor for HValsCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("HValsCommandPara process start: {:?}", &self);

        match data.hash_data.get(&self.key) {
            Some(hash) => {
                let values: Vec<Resp> = hash
                    .values()
                    .map(|v| Resp::BulkStrings(BulkStrings::new(v.clone())))
                    .collect();
                Ok(Resp::Arrays(Arrays::new(values)))
            }
            None => Ok(Resp::Arrays(Arrays::new(Vec::new()))),
        }
    }
}
