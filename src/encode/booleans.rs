use crate::{resp::Booleans, Encoder};

impl Encoder for Booleans {
    fn encode(self) -> Result<Vec<u8>, anyhow::Error> {
        Ok(format!("#{}\r\n", if self.val { "t" } else { "f" })
            .as_bytes()
            .to_vec())
    }
}
// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::Booleans;

    #[test]
    fn test_encode() {
        let b = Booleans { val: true };
        assert_eq!(b.encode().unwrap(), "#t\r\n".as_bytes().to_vec());
    }
}
