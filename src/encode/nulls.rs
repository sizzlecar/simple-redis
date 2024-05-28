use crate::{resp::Nulls, RespEncoder};

impl RespEncoder for Nulls {
    fn encode(self) -> Result<Vec<u8>, anyhow::Error> {
        Ok(b"_-1\r\n".to_vec())
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::Nulls;

    #[test]
    fn test_encode() {
        let n = Nulls::new();
        assert_eq!(n.encode().unwrap(), b"_-1\r\n".to_vec());
    }
}
