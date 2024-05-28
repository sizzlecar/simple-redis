use crate::{resp::BigNumbers, RespEncoder};

impl RespEncoder for BigNumbers {
    fn encode(self) -> Result<Vec<u8>, anyhow::Error> {
        Ok(format!("({}\r\n", self.val).as_bytes().to_vec())
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::BigNumbers;

    #[test]
    fn test_encode_big_numbers() {
        let bn = BigNumbers { val: 1234567890 };
        assert_eq!(bn.encode().unwrap(), b"(1234567890\r\n".to_vec());
    }
}
