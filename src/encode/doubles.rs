use crate::{resp::Doubles, RespEncoder};

impl RespEncoder for Doubles {
    fn encode(self) -> Result<Vec<u8>, anyhow::Error> {
        Ok(format!(",{}\r\n", self.val).as_bytes().to_vec())
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::Doubles;

    #[test]
    fn test_encode() {
        let d = Doubles { val: 13.14 };
        assert_eq!(d.encode().unwrap(), ",13.14\r\n".as_bytes().to_vec());
    }
}
