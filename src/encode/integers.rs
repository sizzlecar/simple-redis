use crate::{resp::Integers, Encoder};

impl Encoder for Integers {
    fn encode(self) -> Result<Vec<u8>, anyhow::Error> {
        Ok(format!(":{}\r\n", self.val).as_bytes().to_vec())
    }
}

//unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::Integers;

    #[test]
    fn test_encode() {
        let i = Integers { val: 123 };
        assert_eq!(i.encode().unwrap(), ":123\r\n".as_bytes().to_vec());
    }
}
