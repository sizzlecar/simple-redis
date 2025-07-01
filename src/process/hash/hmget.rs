use crate::process::Parameter;
use crate::{Data, Processor, Resp};

#[derive(Debug)]
pub struct HMGetCommandPara {
    key: String,
    fields: Vec<String>,
    #[allow(dead_code)]
    parameter: Parameter,
}

impl HMGetCommandPara {
    pub fn new(key: String, fields: Vec<String>, parameter: Parameter) -> Self {
        Self {
            key,
            fields,
            parameter,
        }
    }
}

impl Processor for HMGetCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 检查键是否过期
        if data.is_expired(&self.key) {
            data.remove_key(&self.key);
            // 所有字段都返回null
            let results: Vec<Resp> = self
                .fields
                .iter()
                .map(|_| Resp::Nulls(crate::resp::Nulls::new()))
                .collect();
            return Ok(Resp::Arrays(crate::resp::Arrays::new(results)));
        }

        let mut results = Vec::new();

        if let Some(hash_map) = data.hash_data.get(&self.key) {
            for field in &self.fields {
                match hash_map.get(field) {
                    Some(value) => {
                        results.push(Resp::BulkStrings(crate::resp::BulkStrings::new(
                            value.clone(),
                        )));
                    }
                    None => {
                        results.push(Resp::Nulls(crate::resp::Nulls::new()));
                    }
                }
            }
        } else {
            // 键不存在，所有字段都返回null
            for _ in &self.fields {
                results.push(Resp::Nulls(crate::resp::Nulls::new()));
            }
        }

        Ok(Resp::Arrays(crate::resp::Arrays::new(results)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BulkStrings;
    use std::collections::HashMap;

    #[test]
    fn test_hmget_existing_fields() {
        let data = Data::new();
        
        // 设置测试数据
        let mut hash_map = HashMap::new();
        hash_map.insert("field1".to_string(), "value1".to_string());
        hash_map.insert("field2".to_string(), "value2".to_string());
        hash_map.insert("field3".to_string(), "value3".to_string());
        data.hash_data.insert("test_key".to_string(), hash_map);
        
        let cmd = HMGetCommandPara::new(
            "test_key".to_string(),
            vec!["field1".to_string(), "field2".to_string()],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        if let Resp::Arrays(arr) = result {
            assert_eq!(arr.val.len(), 2);
            
            match &arr.val[0] {
                Resp::BulkStrings(bs) => assert_eq!(bs.val, "value1"),
                _ => panic!("Expected BulkStrings"),
            }
            
            match &arr.val[1] {
                Resp::BulkStrings(bs) => assert_eq!(bs.val, "value2"),
                _ => panic!("Expected BulkStrings"),
            }
        } else {
            panic!("Expected Arrays response");
        }
    }

    #[test]
    fn test_hmget_nonexistent_fields() {
        let data = Data::new();
        
        // 设置测试数据，但不包含查询的字段
        let mut hash_map = HashMap::new();
        hash_map.insert("existing_field".to_string(), "value".to_string());
        data.hash_data.insert("test_key".to_string(), hash_map);
        
        let cmd = HMGetCommandPara::new(
            "test_key".to_string(),
            vec!["nonexistent1".to_string(), "nonexistent2".to_string()],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        if let Resp::Arrays(arr) = result {
            assert_eq!(arr.val.len(), 2);
            
            // 都应该是null
            for item in &arr.val {
                match item {
                    Resp::Nulls(_) => {},
                    _ => panic!("Expected Nulls for nonexistent field"),
                }
            }
        } else {
            panic!("Expected Arrays response");
        }
    }

    #[test]
    fn test_hmget_mixed_fields() {
        let data = Data::new();
        
        let mut hash_map = HashMap::new();
        hash_map.insert("field1".to_string(), "value1".to_string());
        data.hash_data.insert("test_key".to_string(), hash_map);
        
        let cmd = HMGetCommandPara::new(
            "test_key".to_string(),
            vec![
                "field1".to_string(),
                "nonexistent".to_string(),
                "field1".to_string(), // 重复字段
            ],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        if let Resp::Arrays(arr) = result {
            assert_eq!(arr.val.len(), 3);
            
            match &arr.val[0] {
                Resp::BulkStrings(bs) => assert_eq!(bs.val, "value1"),
                _ => panic!("Expected BulkStrings"),
            }
            
            match &arr.val[1] {
                Resp::Nulls(_) => {},
                _ => panic!("Expected Nulls"),
            }
            
            match &arr.val[2] {
                Resp::BulkStrings(bs) => assert_eq!(bs.val, "value1"),
                _ => panic!("Expected BulkStrings"),
            }
        } else {
            panic!("Expected Arrays response");
        }
    }

    #[test]
    fn test_hmget_nonexistent_key() {
        let data = Data::new();
        
        let cmd = HMGetCommandPara::new(
            "nonexistent_key".to_string(),
            vec!["field1".to_string(), "field2".to_string()],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        if let Resp::Arrays(arr) = result {
            assert_eq!(arr.val.len(), 2);
            
            // 都应该是null
            for item in &arr.val {
                match item {
                    Resp::Nulls(_) => {},
                    _ => panic!("Expected Nulls for nonexistent key"),
                }
            }
        } else {
            panic!("Expected Arrays response");
        }
    }

    #[test]
    fn test_hmget_expired_key() {
        let data = Data::new();
        
        // 设置一个已过期的键
        let mut hash_map = HashMap::new();
        hash_map.insert("field1".to_string(), "value1".to_string());
        data.hash_data.insert("expired_key".to_string(), hash_map);
        data.expiry_data.insert("expired_key".to_string(), 1); // 过期时间设为1毫秒（已过期）
        
        let cmd = HMGetCommandPara::new(
            "expired_key".to_string(),
            vec!["field1".to_string()],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        if let Resp::Arrays(arr) = result {
            assert_eq!(arr.val.len(), 1);
            
            match &arr.val[0] {
                Resp::Nulls(_) => {},
                _ => panic!("Expected Nulls for expired key"),
            }
            
            // 验证键已被删除
            assert!(!data.hash_data.contains_key("expired_key"));
        } else {
            panic!("Expected Arrays response");
        }
    }

    #[test]
    fn test_hmget_empty_fields() {
        let data = Data::new();
        
        let mut hash_map = HashMap::new();
        hash_map.insert("field1".to_string(), "value1".to_string());
        data.hash_data.insert("test_key".to_string(), hash_map);
        
        let cmd = HMGetCommandPara::new(
            "test_key".to_string(),
            vec![], // 空字段列表
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        if let Resp::Arrays(arr) = result {
            assert_eq!(arr.val.len(), 0);
        } else {
            panic!("Expected Arrays response");
        }
    }
}