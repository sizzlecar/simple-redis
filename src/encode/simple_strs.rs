use crate::RespEncoder; // Add missing import

use crate::resp::SimpleStringsData;

impl RespEncoder for SimpleStringsData {
    fn encode(self) -> Result<Vec<u8>, anyhow::Error> {
        Ok(format!("+{}\r\n", self.val).as_bytes().to_vec())
    }
}

//unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::SimpleStringsData;

    #[test]
    fn test_encode() {
        let s = SimpleStringsData {
            val: "hello".to_string(),
        };
        assert_eq!(s.encode().unwrap(), "+hello\r\n".as_bytes().to_vec());
    }
}
