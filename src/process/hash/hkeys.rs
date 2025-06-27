use tracing::info;

use crate::{process::Parameter, Arrays, BulkStrings, Data, Processor, Resp};

#[derive(Debug)]
pub struct HKeysCommandPara {
    pub key: String,
    para: Parameter,
}

impl HKeysCommandPara {
    pub fn new(key: String, para: Parameter) -> Self {
        Self { key, para }
    }
}

impl Processor for HKeysCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        info!("HKeysCommandPara process start: {:?}", &self);

        match data.hash_data.get(&self.key) {
            Some(hash) => {
                let keys: Vec<Resp> = hash
                    .keys()
                    .map(|k| Resp::BulkStrings(BulkStrings::new(k.clone())))
                    .collect();
                Ok(Resp::Arrays(Arrays::new(keys)))
            }
            None => Ok(Resp::Arrays(Arrays::new(Vec::new()))),
        }
    }
}
