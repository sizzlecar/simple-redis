use bytes::{Buf, BytesMut};
use tracing::{debug, info, warn};

use crate::{
    resp::Resp, Arrays, BigNumbers, Booleans, BulkStrings, Doubles, Integers, Nulls, RespDecoder,
    RespEncoder, RespError, SimpleErrors, SimpleStringsData, ARRAYS_BYTE, BIG_NUMBERS_BYTE,
    BOOLEANS_BYTE, BULK_STRINGS_BYTE, CRLF, DOUBLES_BYTE, ERRORS_BYTE, INTEGERS_BYTE, NULLS_BYTE,
    SIMPLE_STRINGS_BYTE,
};

impl RespDecoder for Resp {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        debug!("RespDecoder.decode buf length: {}", buf.len());
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
                let str_len = exact_advance(buf, true)?.parse::<isize>()?;
                if str_len < 0 {
                    // 处理 NULL bulk string
                    return Ok(Resp::Nulls(Nulls::new()));
                }
                let str_len = str_len as usize;
                if buf.len() < str_len + CRLF.len() {
                    return Err(RespError::NotComplete);
                }
                let val = buf[..str_len].to_vec();
                buf.advance(str_len + CRLF.len());
                Ok(Resp::BulkStrings(BulkStrings::new(String::from_utf8(val)?)))
            }
            Some(&ARRAYS_BYTE) => {
                let array_len = exact_advance(buf, true)?;
                let array_len: usize = array_len.parse()?;
                let mut arr: Vec<Resp> = Vec::new();
                //array的长度就是需要解析元素的次数
                for i in 0..array_len {
                    debug!("Parsing array element {}/{}", i + 1, array_len);

                    // 检查是否有足够的数据继续解析
                    if buf.is_empty() {
                        return Err(RespError::NotComplete);
                    }

                    arr.push(Resp::decode(buf)?);

                    // 不需要手动advance，decode函数会处理
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
    debug!("exact_advance start, buf length: {}", buf.len());
    let pos_opt = buf
        .windows(CRLF.len())
        .position(|window: &[u8]| window == CRLF);
    if let Some(pos) = pos_opt {
        let res: String = String::from_utf8(buf[1..pos].to_vec())?;
        if advance_flag {
            buf.advance(pos + CRLF.len());
        }
        debug!("exact_advance end, remaining buf length: {}", buf.len());
        Ok(res)
    } else {
        Err(RespError::NotComplete)
    }
}

fn advance_to_next(buf: &mut BytesMut) -> Result<usize, RespError> {
    debug!("advance_to_next start, buf length: {}", buf.len());

    // 检查缓冲区是否为空或太小
    if buf.len() < CRLF.len() {
        return Err(RespError::NotComplete);
    }

    let pos_opt = buf
        .windows(CRLF.len())
        .position(|window: &[u8]| window == CRLF);
    if let Some(pos) = pos_opt {
        let res: usize = pos + CRLF.len();
        debug!("advance_to_next found CRLF at position: {}", res);
        match buf.first() {
            Some(&BULK_STRINGS_BYTE) => {
                let len: String = exact(buf)?;
                let parsed_len = len.parse::<isize>().map_err(RespError::ParseIntError)?;

                if parsed_len < 0 {
                    // NULL bulk string
                    return Ok(res);
                }

                let end = res + parsed_len as usize + 2 * CRLF.len();
                debug!("advance_to_next end, calculated end: {}", end);
                Ok(end)
            }
            _ => {
                let end = res;
                debug!("advance_to_next end, simple end: {}", end);
                Ok(end)
            }
        }
    } else {
        // 不是错误，只是数据不完整
        Err(RespError::NotComplete)
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
