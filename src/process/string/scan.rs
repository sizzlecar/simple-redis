use crate::{process::Parameter, Data, Processor, Resp};

#[derive(Debug)]
pub struct ScanCommandPara {
    cursor: u64,
    pattern: Option<String>,
    count: Option<u64>,
    #[allow(dead_code)]
    para: Parameter,
}

impl ScanCommandPara {
    pub fn new(cursor: u64, pattern: Option<String>, count: Option<u64>, para: Parameter) -> Self {
        Self {
            cursor,
            pattern,
            count,
            para,
        }
    }
}

impl Processor for ScanCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 获取所有键（从所有数据类型中）
        let mut all_keys: Vec<String> = Vec::new();

        // 添加字符串键
        all_keys.extend(data.string_data.iter().map(|entry| entry.key().clone()));

        // 添加哈希键
        all_keys.extend(data.hash_data.iter().map(|entry| entry.key().clone()));

        // 添加列表键
        all_keys.extend(data.list_data.iter().map(|entry| entry.key().clone()));

        // 添加集合键
        all_keys.extend(data.set_data.iter().map(|entry| entry.key().clone()));

        // 添加有序集合键
        all_keys.extend(data.sorted_set_data.iter().map(|entry| entry.key().clone()));

        // 应用模式匹配（如果指定了MATCH参数）
        let matched_keys: Vec<String> = if let Some(pattern) = &self.pattern {
            if pattern == "*" {
                // 匹配所有键
                all_keys
            } else {
                // 简单的模式匹配实现（支持*通配符）
                all_keys
                    .into_iter()
                    .filter(|key| simple_pattern_match(key, pattern))
                    .collect()
            }
        } else {
            all_keys
        };

        // 分页处理
        let count = self.count.unwrap_or(10) as usize;
        let start_index = self.cursor as usize;
        let end_index = std::cmp::min(start_index + count, matched_keys.len());

        let page_keys = if start_index < matched_keys.len() {
            matched_keys[start_index..end_index].to_vec()
        } else {
            Vec::new()
        };

        // 计算下一个游标
        let next_cursor = if end_index >= matched_keys.len() {
            0 // 表示扫描完成
        } else {
            end_index as u64
        };

        // 构建响应数组
        let mut key_responses = Vec::new();
        for key in page_keys {
            key_responses.push(Resp::BulkStrings(crate::BulkStrings { val: key }));
        }

        let response = Resp::Arrays(crate::Arrays {
            val: vec![
                // 下一个游标
                Resp::BulkStrings(crate::BulkStrings {
                    val: next_cursor.to_string(),
                }),
                // 键列表数组
                Resp::Arrays(crate::Arrays { val: key_responses }),
            ],
        });

        Ok(response)
    }
}

// 简单的模式匹配函数，支持*通配符
fn simple_pattern_match(text: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }

    // 简单实现：将*替换为正则表达式的.*
    let regex_pattern = pattern.replace("*", ".*");
    let regex = match regex::Regex::new(&format!("^{regex_pattern}$")) {
        Ok(re) => re,
        Err(_) => return false,
    };

    regex.is_match(text)
}
