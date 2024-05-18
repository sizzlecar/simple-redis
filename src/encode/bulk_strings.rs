use crate::{resp::BulkStrings, Encoder};

impl Encoder for BulkStrings {
    fn encode(self) -> Result<Vec<u8>, anyhow::Error> {
        Ok(format!("${}\r\n{}\r\n", self.val.len(), self.val)
            .as_bytes()
            .to_vec())
    }
}

//unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::BulkStrings;

    #[test]
    fn test_bulk_strings_encode() {
        let bulk_string = BulkStrings {
            val: "hello".to_string(),
        };
        assert_eq!(
            bulk_string.encode().unwrap(),
            "$5\r\nhello\r\n".as_bytes().to_vec()
        );
    }
}
