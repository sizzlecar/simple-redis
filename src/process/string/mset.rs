use crate::process::Parameter;
use crate::{Data, Processor, Resp};

#[derive(Debug)]
pub struct MSetCommandPara {
    key_values: Vec<(String, String)>,
    #[allow(dead_code)]
    parameter: Parameter,
}

impl MSetCommandPara {
    pub fn new(key_values: Vec<(String, String)>, parameter: Parameter) -> Self {
        Self { key_values, parameter }
    }
}

impl Processor for MSetCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // MSET是原子操作，要么全部成功，要么全部失败
        for (key, value) in &self.key_values {
            // 清除过期时间（如果存在）
            data.remove_expiry(key);
            
            // 设置新值
            data.string_data.insert(
                key.clone(),
                Resp::BulkStrings(crate::resp::BulkStrings::new(value.clone())),
            );
        }

        Ok(Resp::SimpleStrings(crate::resp::SimpleStringsData::new(
            "OK".to_string(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BulkStrings;

    #[test]
    fn test_mset_basic() {
        let data = Data::new();
        
        let cmd = MSetCommandPara::new(
            vec![
                ("key1".to_string(), "value1".to_string()),
                ("key2".to_string(), "value2".to_string()),
                ("key3".to_string(), "value3".to_string()),
            ],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        // 验证返回值是OK
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证所有键值都被正确设置
        assert_eq!(data.string_data.len(), 3);
        assert!(data.string_data.contains_key("key1"));
        assert!(data.string_data.contains_key("key2"));
        assert!(data.string_data.contains_key("key3"));
    }

    #[test]
    fn test_mset_overwrite_existing() {
        let data = Data::new();
        
        // 预设一些数据
        data.string_data.insert("key1".to_string(), Resp::BulkStrings(BulkStrings::new("old_value".to_string())));
        
        let cmd = MSetCommandPara::new(
            vec![
                ("key1".to_string(), "new_value".to_string()),
                ("key2".to_string(), "value2".to_string()),
            ],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证键被正确更新
        assert_eq!(data.string_data.len(), 2);
        assert!(data.string_data.contains_key("key1"));
        assert!(data.string_data.contains_key("key2"));
    }

    #[test]
    fn test_mset_clears_expiry() {
        let data = Data::new();
        
        // 设置一个带过期时间的键
        data.string_data.insert("key1".to_string(), Resp::BulkStrings(BulkStrings::new("old_value".to_string())));
        data.expiry_data.insert("key1".to_string(), data.current_timestamp_millis() + 10000); // 10秒后过期
        
        // 验证过期时间已设置
        assert!(data.expiry_data.contains_key("key1"));
        
        let cmd = MSetCommandPara::new(
            vec![("key1".to_string(), "new_value".to_string())],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证键存在
        assert!(data.string_data.contains_key("key1"));
        
        // 验证过期时间被清除
        assert!(!data.expiry_data.contains_key("key1"));
    }

    #[test]
    fn test_mset_empty_pairs() {
        let data = Data::new();
        
        let cmd = MSetCommandPara::new(
            vec![],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证没有键被设置
        assert_eq!(data.string_data.len(), 0);
    }

    #[test]
    fn test_mset_duplicate_keys() {
        let data = Data::new();
        
        // 同一个键设置多次，最后一次应该生效
        let cmd = MSetCommandPara::new(
            vec![
                ("key1".to_string(), "value1".to_string()),
                ("key1".to_string(), "value2".to_string()),
                ("key1".to_string(), "final_value".to_string()),
                ("key2".to_string(), "value2".to_string()),
            ],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证键被设置
        assert_eq!(data.string_data.len(), 2);
        assert!(data.string_data.contains_key("key1"));
        assert!(data.string_data.contains_key("key2"));
    }
}