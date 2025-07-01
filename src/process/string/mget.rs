use crate::process::Parameter;
use crate::{Data, Processor, Resp};

#[derive(Debug)]
pub struct MGetCommandPara {
    keys: Vec<String>,
    #[allow(dead_code)]
    parameter: Parameter,
}

impl MGetCommandPara {
    pub fn new(keys: Vec<String>, parameter: Parameter) -> Self {
        Self { keys, parameter }
    }
}

impl Processor for MGetCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        let mut results = Vec::new();
        
        for key in &self.keys {
            // 检查键是否过期
            if data.is_expired(key) {
                data.remove_key(key);
                results.push(Resp::Nulls(crate::resp::Nulls::new()));
                continue;
            }

            match data.string_data.get(key) {
                Some(value) => results.push(value.clone()),
                None => results.push(Resp::Nulls(crate::resp::Nulls::new())),
            }
        }

        Ok(Resp::Arrays(crate::resp::Arrays::new(results)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BulkStrings;

    #[test]
    fn test_mget_existing_keys() {
        let data = Data::new();
        
        // 设置一些测试数据
        data.string_data.insert("key1".to_string(), Resp::BulkStrings(BulkStrings::new("value1".to_string())));
        data.string_data.insert("key2".to_string(), Resp::BulkStrings(BulkStrings::new("value2".to_string())));
        
        let cmd = MGetCommandPara::new(
            vec!["key1".to_string(), "key2".to_string()],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        if let Resp::Arrays(arr) = result {
            assert_eq!(arr.val.len(), 2);
            // 检查返回值
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
    fn test_mget_nonexistent_keys() {
        let data = Data::new();
        
        let cmd = MGetCommandPara::new(
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
                    _ => panic!("Expected Nulls for nonexistent key"),
                }
            }
        } else {
            panic!("Expected Arrays response");
        }
    }

    #[test]
    fn test_mget_mixed_keys() {
        let data = Data::new();
        
        // 只设置一个键
        data.string_data.insert("key1".to_string(), Resp::BulkStrings(BulkStrings::new("value1".to_string())));
        
        let cmd = MGetCommandPara::new(
            vec!["key1".to_string(), "nonexistent".to_string()],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        if let Resp::Arrays(arr) = result {
            assert_eq!(arr.val.len(), 2);
            // 第一个应该有值
            match &arr.val[0] {
                Resp::BulkStrings(bs) => assert_eq!(bs.val, "value1"),
                _ => panic!("Expected BulkStrings"),
            }
            // 第二个应该是null
            match &arr.val[1] {
                Resp::Nulls(_) => {},
                _ => panic!("Expected Nulls"),
            }
        } else {
            panic!("Expected Arrays response");
        }
    }

    #[test]
    fn test_mget_expired_key() {
        let data = Data::new();
        
        // 设置一个已过期的键
        data.string_data.insert("expired_key".to_string(), Resp::BulkStrings(BulkStrings::new("value".to_string())));
        data.expiry_data.insert("expired_key".to_string(), 1); // 过期时间设为1毫秒（已过期）
        
        let cmd = MGetCommandPara::new(
            vec!["expired_key".to_string()],
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        if let Resp::Arrays(arr) = result {
            assert_eq!(arr.val.len(), 1);
            // 过期的键应该返回null
            match &arr.val[0] {
                Resp::Nulls(_) => {},
                _ => panic!("Expected Nulls for expired key"),
            }
            // 验证键已被删除
            assert!(!data.string_data.contains_key("expired_key"));
        } else {
            panic!("Expected Arrays response");
        }
    }
}