use anyhow::Ok;
use tracing::info;

use crate::{process::Parameter, Data, Processor, Resp, SimpleStringsData};

#[derive(Debug)]
#[allow(unused)]
pub struct SetCommandPara {
    pub key: Option<String>,

    pub value: Option<String>,

    para: Parameter,
}

impl SetCommandPara {
    pub fn new(key: Option<String>, value: Option<String>, para: Parameter) -> Self {
        Self { key, value, para }
    }
}

impl Processor for SetCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        match (&self.key, &self.value) {
            (Some(k), Some(v)) => {
                // 处理特殊的管理命令
                match k.as_str() {
                    "__client__" | "__select__" | "__unsupported__" => {
                        info!("🔧 Management command -> OK");
                        return Ok(Resp::SimpleStrings(SimpleStringsData::new("OK".to_owned())));
                    }
                    _ => {}
                }

                // 正常的SET操作
                data.string_data.insert(
                    k.clone(),
                    Resp::SimpleStrings(SimpleStringsData::new(v.clone())),
                );
                info!("✅ SET '{}' = '{}' -> OK", k, v);
                Ok(Resp::SimpleStrings(SimpleStringsData::new("OK".to_owned())))
            }
            _ => {
                info!("❌ SET -> missing key or value");
                Ok(Resp::SimpleStrings(SimpleStringsData::new(
                    "ERR wrong number of arguments for 'set' command".to_owned(),
                )))
            }
        }
    }
}
