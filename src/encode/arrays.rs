use crate::{resp::Arrays, RespEncoder};

impl RespEncoder for Arrays {
    fn encode(self) -> Result<Vec<u8>, anyhow::Error> {
        let mut encoded = format!("*{}\r\n", self.val.len()).as_bytes().to_vec();
        for val in self.val {
            encoded.extend(val.encode()?);
        }
        Ok(encoded)
    }
}

// unit tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BigNumbers, BulkStrings, Doubles, Integers, Resp};

    #[test]
    fn test_encode_arrays() {
        let arrays = Arrays {
            val: vec![
                Resp::BulkStrings(BulkStrings {
                    val: "foo".to_string(),
                }),
                Resp::Integers(Integers { val: 42 }),
                Resp::Doubles(Doubles { val: 13.14 }),
                Resp::BigNumbers(BigNumbers { val: 123456789 }),
            ],
        };
        let res = arrays.encode().unwrap();
        let expected = "*4\r\n$3\r\nfoo\r\n:42\r\n,13.14\r\n(123456789\r\n".as_bytes();
        assert_eq!(res, expected);
    }
}
