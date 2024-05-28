use crate::{resp::SimpleErrors, RespEncoder};

impl RespEncoder for SimpleErrors {
    fn encode(self) -> Result<Vec<u8>, anyhow::Error> {
        Ok(format!("-{}\r\n", self.error_msg).as_bytes().to_vec())
    }
}

//unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::SimpleErrors;

    #[test]
    fn test_encode() {
        let e = SimpleErrors {
            error_msg: "ERR".to_string(),
        };
        assert_eq!(e.encode().unwrap(), "-ERR\r\n".as_bytes().to_vec());
    }
}
