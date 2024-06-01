use bytes::{Buf, BytesMut};

use crate::{
    resp::Resp, Arrays, BigNumbers, Booleans, BulkStrings, Doubles, Integers, Nulls, RespDecoder,
    RespEncoder, RespError, SimpleErrors, SimpleStringsData, ARRAYS_BYTE, BIG_NUMBERS_BYTE,
    BOOLEANS_BYTE, BULK_STRINGS_BYTE, CRLF, DOUBLES_BYTE, ERRORS_BYTE, INTEGERS_BYTE, NULLS_BYTE,
    SIMPLE_STRINGS_BYTE,
};

impl RespDecoder for Resp {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        println!("decode buf: {:?}", buf);
        //1. 取出第一个byte 判断RESP
        //2. 根据每个RESP的类型的反序列化规则 转化为 对应的RESP类型
        //3. 如果是Array类型 则继续递归解析
        //Clients send commands to a Redis server as an array of bulk strings.
        //The first (and sometimes also the second) bulk string in the array is the command's name.
        //Subsequent elements of the array are the arguments for the command.
        match buf.first() {
            Some(&SIMPLE_STRINGS_BYTE) => Ok(Resp::SimpleStrings(SimpleStringsData::new(
                exact_advance(buf, true)?,
            ))),
            Some(&ERRORS_BYTE) => Ok(Resp::SimpleErrors(SimpleErrors::new(exact_advance(
                buf, true,
            )?))),
            Some(&INTEGERS_BYTE) => {
                let str = exact_advance(buf, true)?;
                str.parse::<i64>()
                    .map(|val| Resp::Integers(Integers::new(val)))
                    .map_err(RespError::ParseIntError)
            }
            Some(&BULK_STRINGS_BYTE) => {
                let str_len = exact_advance(buf, true)?.parse()?;
                let val = buf[..str_len].to_vec();
                buf.advance(str_len + CRLF.len());
                Ok(Resp::BulkStrings(BulkStrings::new(String::from_utf8(val)?)))
            }
            Some(&ARRAYS_BYTE) => {
                let array_len = exact_advance(buf, true)?;
                let mut arr: Vec<Resp> = Vec::new();
                //array的长度就是需要解析元素的次数
                for i in 0..array_len.parse::<usize>()? {
                    println!("i: {}, buf:{:?}", i, buf);
                    arr.push(Resp::decode(buf)?);
                    println!("i: {}, arr:{:?}", i, arr);
                    if !buf.is_empty() {
                        advance_to_next(buf)?;
                    }
                }
                Ok(Resp::Arrays(Arrays::new(arr)))
            }
            Some(&NULLS_BYTE) => {
                let null = Resp::Nulls(Nulls::new());
                let null_encode: Result<Vec<u8>, anyhow::Error> = null.clone().encode();
                buf.advance(
                    null_encode
                        .map_err(|_| {
                            RespError::InternalError("calc null encode len error".to_owned())
                        })?
                        .len(),
                );
                Ok(null)
            }
            Some(&BOOLEANS_BYTE) => {
                let val = exact_advance(buf, true)?;
                if val == "t" {
                    Ok(Resp::Booleans(Booleans::new(true)))
                } else if val == "f" {
                    Ok(Resp::Booleans(Booleans::new(false)))
                } else {
                    Err(RespError::InvalidFrameType(format!(
                        "invalid RESP boolean: {:?}",
                        val
                    )))
                }
            }
            Some(&DOUBLES_BYTE) => {
                let val = exact_advance(buf, true)?;
                val.parse::<f64>()
                    .map(|val| Resp::Doubles(Doubles::new(val)))
                    .map_err(RespError::ParseFloatError)
            }
            Some(&BIG_NUMBERS_BYTE) => {
                let val = exact_advance(buf, true)?;
                val.parse::<i128>()
                    .map(|val| Resp::BigNumbers(BigNumbers::new(val)))
                    .map_err(RespError::ParseIntError)
            }
            None => Err(RespError::NotComplete),
            _ => Err(RespError::InvalidFrameType(format!(
                "expect_length: unknown frame type: {:?}",
                buf
            ))),
        }
    }
}

fn exact(buf: &mut BytesMut) -> Result<String, RespError> {
    exact_advance(buf, false)
}

fn exact_advance(buf: &mut BytesMut, advance_flag: bool) -> Result<String, RespError> {
    println!("exact_advance start: buf:{:?}", buf);
    let pos_opt = buf
        .windows(CRLF.len())
        .position(|window: &[u8]| window == CRLF);
    if let Some(pos) = pos_opt {
        let res: String = String::from_utf8(buf[1..pos].to_vec())?;
        if advance_flag {
            buf.advance(pos + CRLF.len());
        }
        println!("exact_advance end: buf:{:?}", buf);
        Ok(res)
    } else {
        Err(RespError::NotComplete)
    }
}

fn advance_to_next(buf: &mut BytesMut) -> Result<usize, RespError> {
    println!("advance_to_next start: buf:{:?}", buf);
    let pos_opt = buf
        .windows(CRLF.len())
        .position(|window: &[u8]| window == CRLF);
    if let Some(pos) = pos_opt {
        let res: usize = pos + CRLF.len();
        println!("advance_to_next buf:{:?}, CRLF index: {}", buf, res);
        match buf.first() {
            Some(&BULK_STRINGS_BYTE) => {
                let len: String = exact(buf)?;
                let end = res + len.parse::<usize>()? + 2 * CRLF.len();
                println!("advance_to_next end: buf:{:?}", buf);
                Ok(end)
            }
            _ => {
                let end = res;
                println!("advance_to_next end: buf:{:?}", buf);
                Ok(end)
            }
        }
    } else {
        Err(RespError::InvalidFrameLength(buf.len() as isize))
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::Resp;
    use crate::RespEncoder; // Import the trait that contains the `encode` method

    #[test]
    fn test_resp_simple_strings() {
        let resp = Resp::SimpleStrings(SimpleStringsData::new("OK".to_string()));
        let encoded: Vec<u8> = resp.clone().encode().unwrap();
        let mut bm = BytesMut::from(encoded.as_slice());
        let decoded: Resp = Resp::decode(&mut bm).unwrap();
        assert_eq!(resp, decoded);
    }

    #[test]
    fn test_resp_simple_errors() {
        let resp = Resp::SimpleErrors(SimpleErrors::new("ERR".to_string()));
        let encoded = resp.clone().encode().unwrap();
        let decoded: Resp = Resp::decode(&mut BytesMut::from(encoded.as_slice())).unwrap();
        assert_eq!(resp, decoded);
    }

    #[test]
    fn test_resp_integers() {
        let resp = Resp::Integers(Integers::new(123));
        let encoded = resp.clone().encode().unwrap();
        let decoded: Resp = Resp::decode(&mut BytesMut::from(encoded.as_slice())).unwrap();
        assert_eq!(resp, decoded);
    }

    #[test]
    fn test_resp_bulk_strings() {
        let resp = Resp::BulkStrings(BulkStrings::new("foobar".to_string()));
        let encoded = resp.clone().encode().unwrap();
        let decoded: Resp = Resp::decode(&mut BytesMut::from(encoded.as_slice())).unwrap();
        assert_eq!(resp, decoded);
    }

    #[test]
    fn test_resp_arrays() {
        let resp = Resp::Arrays(Arrays::new(vec![
            Resp::SimpleStrings(SimpleStringsData::new("foo".to_string())),
            Resp::SimpleStrings(SimpleStringsData::new("bar".to_string())),
        ]));
        let decoded: Resp =
            Resp::decode(&mut BytesMut::from(b"*2\r\n+foo\r\n+bar\r\n".as_slice())).unwrap();
        assert_eq!(resp, decoded);
    }

    #[test]
    fn test_resp_nulls() {
        let resp = Resp::Nulls(Nulls::new());
        let encoded = resp.clone().encode().unwrap();
        let decoded: Resp = Resp::decode(&mut BytesMut::from(encoded.as_slice())).unwrap();
        assert_eq!(resp, decoded);
    }

    #[test]
    fn test_resp_booleans() {
        let resp = Resp::Booleans(Booleans::new(true));
        let encoded = resp.clone().encode().unwrap();
        let decoded: Resp = Resp::decode(&mut BytesMut::from(encoded.as_slice())).unwrap();
        assert_eq!(resp, decoded);
    }

    #[test]
    fn test_resp_doubles() {
        let resp = Resp::Doubles(Doubles::new(13.14));
        let encoded = resp.clone().encode().unwrap();
        let decoded: Resp = Resp::decode(&mut BytesMut::from(encoded.as_slice())).unwrap();
        assert_eq!(resp, decoded);
    }

    #[test]
    fn test_resp_big_numbers() {
        let resp = Resp::BigNumbers(BigNumbers::new(123456789012345678901234567890));
        let encoded = resp.clone().encode().unwrap();
        let decoded: Resp = Resp::decode(&mut BytesMut::from(encoded.as_slice())).unwrap();
        assert_eq!(resp, decoded);
    }

    #[test]
    fn test_array_bulk_strings() {
        let decoded: Resp = Resp::decode(&mut BytesMut::from(
            b"*2\r\n$7\r\nCOMMAND\r\n$4\r\nDOCS\r\n".as_slice(),
        ))
        .unwrap();
        let resp = Resp::Arrays(Arrays::new(vec![
            Resp::BulkStrings(BulkStrings::new("COMMAND".to_string())),
            Resp::BulkStrings(BulkStrings::new("DOCS".to_string())),
        ]));
        assert_eq!(resp, decoded);
    }
}
