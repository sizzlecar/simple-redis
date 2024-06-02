use enum_dispatch::enum_dispatch;
use tracing::info;

use crate::process::string::StringCommand;
use crate::{GetCommandPara, Resp};

use self::string::set::SetCommandPara;
use std::convert::TryFrom;

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

impl Default for Parameter {
    fn default() -> Self {
        Self::new()
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

impl TryFrom<Resp> for CommandGroup {
    type Error = anyhow::Error;

    fn try_from(value: Resp) -> Result<Self, Self::Error> {
        println!("value: {:?}", &value);
        match value {
            Resp::Arrays(arr) => {
                let mut iter = arr.val.iter();
                let command = try_exact_bulk_string(iter.next())?;
                println!("command: {:?}", &command);
                match command.to_lowercase().as_str() {
                    "set" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let value = try_exact_bulk_string(iter.next())?;
                        let mut para = Parameter::new();
                        for item in iter {
                            let key = try_exact_bulk_string(Some(item))?;
                            para.add(key.to_string(), None);
                        }
                        info!("key:{}, value:{} para: {:?}", key, value, &para);
                        Ok(CommandGroup::String(StringCommand::Set(
                            SetCommandPara::new(
                                Some(key.to_string()),
                                Some(value.to_string()),
                                para,
                            ),
                        )))
                    }
                    "get" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        info!("key:{}", key);
                        Ok(CommandGroup::String(StringCommand::Get(
                            GetCommandPara::new(Some(key.to_string()), None, Parameter::new()),
                        )))
                    }
                    _ => Err(anyhow::anyhow!("not support command")),
                }
            }
            _ => Err(anyhow::anyhow!("unsupported command")),
        }
    }
}

//断言resp类型为bulk string，返回值，其他的类型视为异常
pub fn try_exact_bulk_string(resp_opt: Option<&Resp>) -> Result<&str, anyhow::Error> {
    match resp_opt {
        Some(Resp::BulkStrings(para)) => {
            info!("para: {:?}", para);
            Ok(para.val.as_str())
        }
        _ => Err(anyhow::anyhow!("invalid command")),
    }
}
