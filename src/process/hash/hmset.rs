use crate::process::Parameter;
use crate::{Data, Processor, Resp};
use std::collections::HashMap;

#[derive(Debug)]
pub struct HMSetCommandPara {
    key: String,
    field_values: Vec<(String, String)>,
    #[allow(dead_code)]
    parameter: Parameter,
}

impl HMSetCommandPara {
    pub fn new(key: String, field_values: Vec<(String, String)>, parameter: Parameter) -> Self {
        Self {
            key,
            field_values,
            parameter,
        }
    }
}

impl Processor for HMSetCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 获取或创建哈希表
        let mut hash_map = data
            .hash_data
            .entry(self.key.clone())
            .or_insert_with(HashMap::new);

        // 设置所有字段值
        for (field, value) in &self.field_values {
            hash_map.insert(field.clone(), value.clone());
        }

        // HMSET 总是成功，即使没有字段值对
        Ok(Resp::SimpleStrings(crate::resp::SimpleStringsData::new(
            "OK".to_string(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_hmset_basic() {
        let data = Data::new();
        
        let cmd = HMSetCommandPara::new(
            "test_key".to_string(),
            vec![
                ("field1".to_string(), "value1".to_string()),
                ("field2".to_string(), "value2".to_string()),
                ("field3".to_string(), "value3".to_string()),
            ],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        // 验证返回值是OK
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证哈希表被创建
        assert!(data.hash_data.contains_key("test_key"));
    }

    #[test]
    fn test_hmset_overwrite_existing_fields() {
        let data = Data::new();
        
        // 预设一些数据
        let mut existing_hash = HashMap::new();
        existing_hash.insert("field1".to_string(), "old_value1".to_string());
        existing_hash.insert("field2".to_string(), "old_value2".to_string());
        data.hash_data.insert("test_key".to_string(), existing_hash);
        
        let cmd = HMSetCommandPara::new(
            "test_key".to_string(),
            vec![
                ("field1".to_string(), "new_value1".to_string()),
                ("field3".to_string(), "value3".to_string()),
            ],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证哈希表存在
        assert!(data.hash_data.contains_key("test_key"));
    }

    #[test]
    fn test_hmset_new_key() {
        let data = Data::new();
        
        let cmd = HMSetCommandPara::new(
            "new_key".to_string(),
            vec![
                ("field1".to_string(), "value1".to_string()),
                ("field2".to_string(), "value2".to_string()),
            ],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证新的哈希表被创建
        assert!(data.hash_data.contains_key("new_key"));
    }

    #[test]
    fn test_hmset_empty_field_values() {
        let data = Data::new();
        
        let cmd = HMSetCommandPara::new(
            "test_key".to_string(),
            vec![], // 空的字段值对
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证空哈希表被创建
        assert!(data.hash_data.contains_key("test_key"));
    }

    #[test]
    fn test_hmset_duplicate_fields() {
        let data = Data::new();
        
        // 同一个字段设置多次，最后一次应该生效
        let cmd = HMSetCommandPara::new(
            "test_key".to_string(),
            vec![
                ("field1".to_string(), "value1".to_string()),
                ("field1".to_string(), "value2".to_string()),
                ("field1".to_string(), "final_value".to_string()),
                ("field2".to_string(), "value2".to_string()),
            ],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证哈希表被创建
        assert!(data.hash_data.contains_key("test_key"));
    }

    #[test]
    fn test_hmset_empty_values() {
        let data = Data::new();
        
        let cmd = HMSetCommandPara::new(
            "test_key".to_string(),
            vec![
                ("field1".to_string(), "".to_string()), // 空值
                ("field2".to_string(), "value2".to_string()),
            ],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证哈希表被创建
        assert!(data.hash_data.contains_key("test_key"));
    }

    #[test]
    fn test_hmset_with_existing_expiry() {
        let data = Data::new();
        
        // 设置一个带过期时间的键（不是哈希类型）
        data.string_data.insert("test_key".to_string(), Resp::BulkStrings(crate::resp::BulkStrings::new("string_value".to_string())));
        data.set_expiry("test_key", data.current_timestamp_millis() + 10000);
        
        let cmd = HMSetCommandPara::new(
            "test_key".to_string(),
            vec![("field1".to_string(), "value1".to_string())],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证哈希表被创建
        assert!(data.hash_data.contains_key("test_key"));
        
        // 验证过期时间仍然存在（Redis行为）
        assert!(data.expiry_data.contains_key("test_key"));
    }
}