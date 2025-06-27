use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

use crate::{process::Parameter, BulkStrings, Data, Processor, Resp};

#[derive(Debug)]
pub struct InfoCommandPara {
    pub section: Option<String>,
    #[allow(dead_code)]
    para: Parameter,
}

impl InfoCommandPara {
    pub fn new(section: Option<String>, para: Parameter) -> Self {
        Self { section, para }
    }
}

impl Processor for InfoCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        let uptime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // 统计数据
        let string_keys = data.string_data.len();
        let hash_keys = data.hash_data.len();
        let list_keys = data.list_data.len();
        let set_keys = data.set_data.len();
        let sorted_set_keys = data.sorted_set_data.len();
        let total_keys = string_keys + hash_keys + list_keys + set_keys + sorted_set_keys;

        let info_text = match self.section.as_deref() {
            Some("server") | None => {
                format!(
                    "# Server\r\n\
                    redis_version:7.0.0-simple\r\n\
                    redis_mode:standalone\r\n\
                    arch_bits:64\r\n\
                    uptime_in_seconds:{uptime}\r\n\
                    \r\n\
                    # Keyspace\r\n\
                    db0:keys={total_keys},expires=0,avg_ttl=0\r\n\
                    \r\n\
                    # Stats\r\n\
                    total_connections_received:1\r\n\
                    total_commands_processed:1\r\n\
                    \r\n\
                    # Memory\r\n\
                    used_memory:1024\r\n\
                    used_memory_human:1.00K\r\n"
                )
            }
            Some("keyspace") => {
                format!(
                    "# Keyspace\r\n\
                    db0:keys={total_keys},expires=0,avg_ttl=0\r\n"
                )
            }
            Some("stats") => "# Stats\r\n\
                    total_connections_received:1\r\n\
                    total_commands_processed:1\r\n"
                .to_string(),
            Some(section) => {
                format!("# {section}\r\n")
            }
        };

        info!(
            "ℹ️ INFO {} -> {} bytes",
            self.section.as_deref().unwrap_or("all"),
            info_text.len()
        );

        Ok(Resp::BulkStrings(BulkStrings::new(info_text)))
    }
}
