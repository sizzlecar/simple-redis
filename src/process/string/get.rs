use tracing::info;

use crate::{process::Parameter, Data, Nulls, Processor, Resp, SimpleErrors};

#[derive(Debug)]
#[allow(unused)]
pub struct GetCommandPara {
    pub key: Option<String>,

    pub value: Option<String>,

    #[allow(dead_code)]
    para: Parameter,
}

impl GetCommandPara {
    pub fn new(key: Option<String>, value: Option<String>, para: Parameter) -> Self {
        Self { key, value, para }
    }
}

impl Processor for GetCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        match &self.key {
            Some(k) => {
                // 处理特殊的管理命令
                match k.as_str() {
                    "__ping__" => {
                        info!("🏓 PING -> PONG");
                        return Ok(Resp::SimpleStrings(crate::SimpleStringsData::new(
                            "PONG".to_owned(),
                        )));
                    }
                    "__command__" => {
                        info!("📋 COMMAND -> empty array");
                        return Ok(Resp::Arrays(crate::Arrays::new(Vec::new())));
                    }

                    _ => {}
                }

                // 检查键是否过期
                if data.is_expired(k) {
                    info!("⏰ GET '{}' -> expired, removing", k);
                    data.remove_key(k);
                    return Ok(Resp::Nulls(Nulls::new()));
                }

                // 正常的GET操作
                match data.string_data.get(k) {
                    Some(value) => {
                        info!("✅ GET '{}' -> found: {:?}", k, value);
                        Ok(value.clone())
                    }
                    None => {
                        info!("❌ GET '{}' -> not found (NULL)", k);
                        Ok(Resp::Nulls(Nulls::new()))
                    }
                }
            }
            None => {
                info!("❌ GET -> missing key");
                Ok(Resp::SimpleErrors(SimpleErrors::new(
                    "ERR wrong number of arguments for 'get' command".to_owned(),
                )))
            }
        }
    }
}
