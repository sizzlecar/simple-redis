use bytes::BytesMut;

use crate::{
    resp::Resp, Arrays, BigNumbers, Booleans, BulkStrings, Doubles, Integers, Nulls, RespDecoder,
    SimpleErrors, SimpleStringsData, ARRAYS_BYTE, BIG_NUMBERS_BYTE, BOOLEANS_BYTE,
    BULK_STRINGS_BYTE, CRLF, DOUBLES_BYTE, ERRORS_BYTE, INTEGERS_BYTE, NULLS_BYTE,
    SIMPLE_STRINGS_BYTE,
};

impl RespDecoder for Resp {
    fn decode(buf: &mut BytesMut) -> Result<Self, anyhow::Error> {
        let item = buf.to_vec();
        //1. 取出第一个byte 判断RESP
        //2. 根据每个RESP的类型的反序列化规则 转化为 对应的RESP类型
        //3. 如果是Array类型 则继续递归解析
        //Clients send commands to a Redis server as an array of bulk strings.
        //The first (and sometimes also the second) bulk string in the array is the command's name.
        //Subsequent elements of the array are the arguments for the command.
        match item.first() {
            Some(&SIMPLE_STRINGS_BYTE) => {
                Ok(Resp::SimpleStrings(SimpleStringsData::new(exact(&item)?)))
            }
            Some(&ERRORS_BYTE) => Ok(Resp::SimpleErrors(SimpleErrors::new(exact(&item)?))),
            Some(&INTEGERS_BYTE) => {
                let str = exact(&item)?;
                str.parse::<i64>()
                    .map(|val| Resp::Integers(Integers::new(val)))
                    .map_err(|_e| anyhow::Error::msg(_e.to_string()))
            }
            Some(&BULK_STRINGS_BYTE) => {
                let vec = vec_split(&item)?;
                if vec.len() != 2 || vec[0].parse::<usize>()? != vec[1].len() {
                    Err(anyhow::Error::msg("invalid RESP bulk string"))
                } else {
                    Ok(Resp::BulkStrings(BulkStrings::new(vec[1].to_string())))
                }
            }
            Some(&ARRAYS_BYTE) => {
                let array_len = exact(&item)?;
                let mut arr: Vec<Resp> = Vec::new();
                //array的长度就是需要解析元素的次数
                let mut start = first_type_end_index(&item)?;
                let mut end = start + first_type_end_index(&item[start..])?;
                for i in 0..array_len.parse::<usize>()? {
                    println!("i: {} start: {:?} end: {:?}", i, start, end);
                    let frag = &item[start..end];
                    let mut bm = BytesMut::from(frag);
                    arr.push(Resp::decode(&mut bm)?);
                    //下一个resp type 的报文开始位置
                    start = end;
                    if start >= item.len() {
                        break;
                    }
                    end = start + first_type_end_index(&item[start..])?;
                }
                Ok(Resp::Arrays(Arrays::new(arr)))
            }
            Some(&NULLS_BYTE) => Ok(Resp::Nulls(Nulls::new())),
            Some(&BOOLEANS_BYTE) => {
                let val = exact(&item)?;
                if val == "t" {
                    Ok(Resp::Booleans(Booleans::new(true)))
                } else if val == "f" {
                    Ok(Resp::Booleans(Booleans::new(false)))
                } else {
                    Err(anyhow::Error::msg("invalid RESP boolean"))
                }
            }
            Some(&DOUBLES_BYTE) => {
                let val = exact(&item)?;
                val.parse::<f64>()
                    .map(|val| Resp::Doubles(Doubles::new(val)))
                    .map_err(|_e| anyhow::Error::msg("invalid RESP double"))
            }
            Some(&BIG_NUMBERS_BYTE) => {
                let val = exact(&item)?;
                val.parse::<i128>()
                    .map(|val| Resp::BigNumbers(BigNumbers::new(val)))
                    .map_err(|_e| anyhow::Error::msg("invalid RESP double"))
            }
            _ => Err(anyhow::Error::msg("unsupported RESP type")),
        }
    }
}

fn exact(item: &[u8]) -> Result<String, anyhow::Error> {
    let pos_opt = item
        .windows(CRLF.len())
        .position(|window: &[u8]| window == CRLF);
    if let Some(pos) = pos_opt {
        let res: String = String::from_utf8(item[1..pos].to_vec())?;
        println!("item:{:?} val: {:?}", String::from_utf8(item.to_vec()), res);
        Ok(res)
    } else {
        Err(anyhow::Error::msg("invalid RESP"))
    }
}

fn first_type_end_index(item: &[u8]) -> Result<usize, anyhow::Error> {
    let pos_opt = item
        .windows(CRLF.len())
        .position(|window: &[u8]| window == CRLF);
    if let Some(pos) = pos_opt {
        let res: usize = pos + CRLF.len();
        println!(
            "item:{:?} index: {:?}",
            String::from_utf8(item.to_vec()),
            res
        );
        Ok(res)
    } else {
        Err(anyhow::Error::msg("invalid RESP"))
    }
}

fn vec_split(item: &[u8]) -> Result<Vec<String>, anyhow::Error> {
    let mut parts = Vec::new();
    let mut start = 1;
    while let Some(end) = item[start..]
        .windows(CRLF.len())
        .position(|window| window == CRLF)
    {
        let part = String::from_utf8(item[start..start + end].to_vec())
            .map_err(|_| anyhow::Error::msg("Invalid UTF-8 string"))?;
        parts.push(part);
        start += end + CRLF.len();
    }

    if start < CRLF.len() {
        let part = String::from_utf8(item[start..].to_vec())
            .map_err(|_| anyhow::Error::msg("Invalid UTF-8 string"))?;
        parts.push(part);
    }

    Ok(parts)
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
        let encoded = resp.clone().encode().unwrap();
        let decoded: Resp = Resp::decode(&mut BytesMut::from(encoded.as_slice())).unwrap();
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
}
