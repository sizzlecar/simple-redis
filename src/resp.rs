//Redis serialization protocol data type enum

use thiserror::Error;

pub const CRLF: [u8; 2] = [b'\r', b'\n'];
pub const SIMPLE_STRINGS_BYTE: u8 = b'+';
pub const ERRORS_BYTE: u8 = b'-';
pub const INTEGERS_BYTE: u8 = b':';
pub const BULK_STRINGS_BYTE: u8 = b'$';
pub const ARRAYS_BYTE: u8 = b'*';
pub const NULLS_BYTE: u8 = b'_';
pub const BOOLEANS_BYTE: u8 = b'#';
pub const DOUBLES_BYTE: u8 = b',';
pub const BIG_NUMBERS_BYTE: u8 = b'(';

#[derive(Debug, PartialEq, Clone)]
pub enum Resp {
    SimpleStrings(SimpleStringsData),
    SimpleErrors(SimpleErrors),
    Integers(Integers),
    BulkStrings(BulkStrings),
    Arrays(Arrays),
    Nulls(Nulls),
    Booleans(Booleans),
    Doubles(Doubles),
    BigNumbers(BigNumbers),
}

// 手动实现RespEncoder for Resp
impl crate::RespEncoder for Resp {
    fn encode(self) -> Result<Vec<u8>, anyhow::Error> {
        match self {
            Resp::SimpleStrings(data) => data.encode(),
            Resp::SimpleErrors(data) => data.encode(),
            Resp::Integers(data) => data.encode(),
            Resp::BulkStrings(data) => data.encode(),
            Resp::Arrays(data) => data.encode(),
            Resp::Nulls(data) => data.encode(),
            Resp::Booleans(data) => data.encode(),
            Resp::Doubles(data) => data.encode(),
            Resp::BigNumbers(data) => data.encode(),
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RespError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),
    #[error("Invalid frame length： {0}")]
    InvalidFrameLength(isize),
    #[error("Frame is not complete")]
    NotComplete,
    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Parse float error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Parse frame error: {0}")]
    InternalError(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleStringsData {
    pub val: String,
}

impl SimpleStringsData {
    pub fn new(val: String) -> Self {
        Self { val }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleErrors {
    pub error_msg: String,
}

impl SimpleErrors {
    pub fn new(error_msg: String) -> Self {
        Self { error_msg }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Integers {
    pub val: i64,
}

impl Integers {
    pub fn new(val: i64) -> Self {
        Self { val }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BulkStrings {
    pub val: String,
}

impl BulkStrings {
    pub fn new(val: String) -> Self {
        Self { val }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Arrays {
    pub val: Vec<Resp>,
}

impl Arrays {
    pub fn new(val: Vec<Resp>) -> Self {
        Self { val }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Nulls {
    pub val: (),
}

impl Default for Nulls {
    fn default() -> Self {
        Self::new()
    }
}

impl Nulls {
    pub fn new() -> Self {
        Self { val: () }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Booleans {
    pub val: bool,
}

impl Booleans {
    pub fn new(val: bool) -> Self {
        Self { val }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Doubles {
    pub val: f64,
}

impl Doubles {
    pub fn new(val: f64) -> Self {
        Self { val }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BigNumbers {
    pub val: i128,
}

impl BigNumbers {
    pub fn new(val: i128) -> Self {
        Self { val }
    }
}
