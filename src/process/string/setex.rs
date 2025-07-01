use crate::process::Parameter;
use crate::{Data, Processor, Resp};

#[derive(Debug)]
pub struct SetExCommandPara {
    key: String,
    seconds: u64,
    value: String,
    #[allow(dead_code)]
    parameter: Parameter,
}

impl SetExCommandPara {
    pub fn new(key: String, seconds: u64, value: String, parameter: Parameter) -> Self {
        Self {
            key,
            seconds,
            value,
            parameter,
        }
    }
}

impl Processor for SetExCommandPara {
    fn process(&self, data: &Data) -> Result<Resp, anyhow::Error> {
        // 设置键值
        data.string_data.insert(
            self.key.clone(),
            Resp::BulkStrings(crate::resp::BulkStrings::new(self.value.clone())),
        );

        // 设置过期时间（秒转换为毫秒）
        let expiry_millis = data.current_timestamp_millis() + (self.seconds * 1000);
        data.set_expiry(&self.key, expiry_millis);

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
    fn test_setex_basic() {
        let data = Data::new();
        
        let cmd = SetExCommandPara::new(
            "test_key".to_string(),
            10, // 10秒
            "test_value".to_string(),
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        // 验证返回值是OK
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证键值被设置
        assert!(data.string_data.contains_key("test_key"));
        
        // 验证过期时间被设置
        assert!(data.expiry_data.contains_key("test_key"));
        
        // 验证TTL大致正确（应该接近10秒，允许一些误差）
        if let Some(ttl) = data.get_ttl_millis("test_key") {
            assert!(ttl > 9000 && ttl <= 10000); // 9-10秒之间
        } else {
            panic!("TTL not found");
        }
    }

    #[test]
    fn test_setex_overwrite_existing() {
        let data = Data::new();
        
        // 先设置一个键
        data.string_data.insert("test_key".to_string(), Resp::BulkStrings(BulkStrings::new("old_value".to_string())));
        
        let cmd = SetExCommandPara::new(
            "test_key".to_string(),
            5, // 5秒
            "new_value".to_string(),
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证键存在
        assert!(data.string_data.contains_key("test_key"));
        
        // 验证过期时间被设置
        if let Some(ttl) = data.get_ttl_millis("test_key") {
            assert!(ttl > 4000 && ttl <= 5000); // 4-5秒之间
        } else {
            panic!("TTL not found");
        }
    }

    #[test]
    fn test_setex_replace_existing_expiry() {
        let data = Data::new();
        
        // 先设置一个带长过期时间的键
        data.string_data.insert("test_key".to_string(), Resp::BulkStrings(BulkStrings::new("old_value".to_string())));
        data.set_expiry("test_key", data.current_timestamp_millis() + 60000); // 60秒
        
        // 用SETEX设置更短的过期时间
        let cmd = SetExCommandPara::new(
            "test_key".to_string(),
            2, // 2秒
            "new_value".to_string(),
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证新的过期时间
        if let Some(ttl) = data.get_ttl_millis("test_key") {
            assert!(ttl > 1000 && ttl <= 2000); // 1-2秒之间，而不是原来的60秒
        } else {
            panic!("TTL not found");
        }
    }

    #[test]
    fn test_setex_zero_seconds() {
        let data = Data::new();
        
        let cmd = SetExCommandPara::new(
            "test_key".to_string(),
            0, // 0秒，应该立即过期
            "test_value".to_string(),
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 键应该被设置
        assert!(data.string_data.contains_key("test_key"));
        
        // 但是应该立即过期
        assert!(data.is_expired("test_key"));
    }

    #[test]
    fn test_setex_large_seconds() {
        let data = Data::new();
        
        let cmd = SetExCommandPara::new(
            "test_key".to_string(),
            3600, // 1小时
            "test_value".to_string(),
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证TTL大致正确
        if let Some(ttl) = data.get_ttl_millis("test_key") {
            assert!(ttl > 3590000 && ttl <= 3600000); // 接近1小时（3600秒）
        } else {
            panic!("TTL not found");
        }
    }

    #[test]
    fn test_setex_empty_value() {
        let data = Data::new();
        
        let cmd = SetExCommandPara::new(
            "test_key".to_string(),
            10,
            "".to_string(), // 空值
            Parameter::new(),
        );
        
        let result = cmd.process(&data).unwrap();
        
        match result {
            Resp::SimpleStrings(ss) => assert_eq!(ss.val, "OK"),
            _ => panic!("Expected SimpleStrings OK response"),
        }
        
        // 验证键被设置
        assert!(data.string_data.contains_key("test_key"));
        
        // 验证过期时间仍然被设置
        assert!(data.expiry_data.contains_key("test_key"));
    }
}