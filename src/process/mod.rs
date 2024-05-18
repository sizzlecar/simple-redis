use enum_dispatch::enum_dispatch;

use crate::process::string::StringCommand;
use crate::{GetCommandPara, Processor, Resp};

use self::string::set::SetCommandPara;

pub mod hash;
pub mod list;
pub mod set;
pub mod sorted_set;
pub mod string;

#[derive(Debug)]
pub struct Parameter {
    entries: Vec<Options>,
}

#[allow(unused)]
impl Parameter {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, k: String, v: Option<String>) {
        self.entries.push(Options::new(k, v));
    }

    pub fn get(&self) -> &Vec<Options> {
        &self.entries
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Options {
    k: String,

    v: Option<String>,
}

impl Options {
    pub fn new(k: String, v: Option<String>) -> Self {
        Self { k, v }
    }
}

// 支持的命令group
#[derive(Debug)]
#[enum_dispatch(Processor)]
pub enum CommandGroup {
    String(StringCommand),
    //Hash(HashCommand),
    //List(ListCommand),
    //Set(SetCommand),
    //SortedSet(SortedSetCommand),
}

//Resp -> Processor
// 1. 从bulk string array 中第一个元素获取命令名
// 2. 从bulk string array 其他元素获取参数
impl std::convert::TryFrom<Resp> for Box<dyn Processor> {
    type Error = anyhow::Error;

    fn try_from(value: Resp) -> Result<Self, Self::Error> {
        match value {
            // 客户端会发送一个bulk 数组，数组的第一个元素是命令名，后面的元素是参数
            Resp::Arrays(arr) => {
                let mut iter = arr.val.iter();
                let command = try_exact_bulk_string(iter.next())?;

                match command.to_lowercase().as_str() {
                    "set" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let value = try_exact_bulk_string(iter.next())?;
                        let mut para = Parameter::new();
                        for item in iter {
                            let key = try_exact_bulk_string(Some(item))?;
                            para.add(key.to_string(), None);
                        }
                        Ok(Box::new(CommandGroup::String(StringCommand::Set(
                            SetCommandPara::new(
                                Some(key.to_string()),
                                Some(value.to_string()),
                                para,
                            ),
                        ))))
                    }
                    "get" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let value = try_exact_bulk_string(iter.next())?;
                        let mut para = Parameter::new();
                        for item in iter {
                            let key = try_exact_bulk_string(Some(item))?;
                            para.add(key.to_string(), None);
                        }
                        Ok(Box::new(CommandGroup::String(StringCommand::Get(
                            GetCommandPara::new(
                                Some(key.to_string()),
                                Some(value.to_string()),
                                para,
                            ),
                        ))))
                    }
                    _ => Err(anyhow::anyhow!("not support command")),
                }
            }
            //其余情况视为异常
            _ => Err(anyhow::anyhow!("unsupported command")),
        }
    }
}

//断言resp类型为bulk string，返回值，其他的类型视为异常
pub fn try_exact_bulk_string(resp_opt: Option<&Resp>) -> Result<&str, anyhow::Error> {
    match resp_opt {
        Some(Resp::BulkStrings(para)) => Ok(para.val.as_str()),
        _ => Err(anyhow::anyhow!("invalid command")),
    }
}
