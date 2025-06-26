use tracing::{error, info};

use crate::process::string::StringCommand;
use crate::process::hash::HashCommand;
use crate::process::list::ListCommand;
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
pub enum CommandGroup {
    String(StringCommand),
    Hash(HashCommand),
    List(ListCommand),
    //Set(SetCommand),
    //SortedSet(SortedSetCommand),
}

impl TryFrom<Resp> for CommandGroup {
    type Error = anyhow::Error;

    fn try_from(value: Resp) -> Result<Self, Self::Error> {
        info!("TryFrom<Resp>.try_from value: {:?}", &value);
        match value {
            Resp::Arrays(arr) => {
                let mut iter = arr.val.iter();
                let command = try_exact_bulk_string(iter.next())?;
                info!("TryFrom<Resp>.try_from command: {:?}", &command);
                match command.to_lowercase().as_str() {
                    // String commands
                    "set" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let value = try_exact_bulk_string(iter.next())?;
                        let mut para = Parameter::new();
                        for item in iter {
                            let key = try_exact_bulk_string(Some(item))?;
                            para.add(key.to_string(), None);
                        }
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
                        Ok(CommandGroup::String(StringCommand::Get(
                            GetCommandPara::new(Some(key.to_string()), None, Parameter::new()),
                        )))
                    }
                    "del" => {
                        let mut keys = Vec::new();
                        for item in iter {
                            let key = try_exact_bulk_string(Some(item))?;
                            keys.push(key.to_string());
                        }
                        Ok(CommandGroup::String(StringCommand::Del(
                            crate::process::string::del::DelCommandPara::new(keys, Parameter::new()),
                        )))
                    }
                    "exists" => {
                        let mut keys = Vec::new();
                        for item in iter {
                            let key = try_exact_bulk_string(Some(item))?;
                            keys.push(key.to_string());
                        }
                        Ok(CommandGroup::String(StringCommand::Exists(
                            crate::process::string::exists::ExistsCommandPara::new(keys, Parameter::new()),
                        )))
                    }
                    "incr" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::String(StringCommand::Incr(
                            crate::process::string::incr::IncrCommandPara::new(key.to_string(), None, Parameter::new()),
                        )))
                    }
                    "decr" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::String(StringCommand::Decr(
                            crate::process::string::decr::DecrCommandPara::new(key.to_string(), None, Parameter::new()),
                        )))
                    }
                    
                    // Hash commands
                    "hset" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let mut field_values = Vec::new();
                        
                        let fields_and_values: Vec<&str> = iter.map(|item| try_exact_bulk_string(Some(item))).collect::<Result<Vec<_>, _>>()?;
                        
                        if fields_and_values.len() % 2 != 0 {
                            return Err(anyhow::anyhow!("wrong number of arguments for HSET"));
                        }
                        
                        for chunk in fields_and_values.chunks(2) {
                            field_values.push((chunk[0].to_string(), chunk[1].to_string()));
                        }
                        
                        Ok(CommandGroup::Hash(HashCommand::HSet(
                            crate::process::hash::hset::HSetCommandPara::new(key.to_string(), field_values, Parameter::new()),
                        )))
                    }
                    "hget" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let field = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::Hash(HashCommand::HGet(
                            crate::process::hash::hget::HGetCommandPara::new(key.to_string(), field.to_string(), Parameter::new()),
                        )))
                    }
                    "hdel" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let mut fields = Vec::new();
                        for item in iter {
                            let field = try_exact_bulk_string(Some(item))?;
                            fields.push(field.to_string());
                        }
                        Ok(CommandGroup::Hash(HashCommand::HDel(
                            crate::process::hash::hdel::HDelCommandPara::new(key.to_string(), fields, Parameter::new()),
                        )))
                    }
                    "hgetall" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::Hash(HashCommand::HGetAll(
                            crate::process::hash::hgetall::HGetAllCommandPara::new(key.to_string(), Parameter::new()),
                        )))
                    }
                    "hkeys" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::Hash(HashCommand::HKeys(
                            crate::process::hash::hkeys::HKeysCommandPara::new(key.to_string(), Parameter::new()),
                        )))
                    }
                    "hvals" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::Hash(HashCommand::HVals(
                            crate::process::hash::hvals::HValsCommandPara::new(key.to_string(), Parameter::new()),
                        )))
                    }
                    
                    // List commands
                    "lpush" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let mut values = Vec::new();
                        for item in iter {
                            let value = try_exact_bulk_string(Some(item))?;
                            values.push(value.to_string());
                        }
                        Ok(CommandGroup::List(ListCommand::LPush(
                            crate::process::list::lpush::LPushCommandPara::new(key.to_string(), values, Parameter::new()),
                        )))
                    }
                    "rpush" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let mut values = Vec::new();
                        for item in iter {
                            let value = try_exact_bulk_string(Some(item))?;
                            values.push(value.to_string());
                        }
                        Ok(CommandGroup::List(ListCommand::RPush(
                            crate::process::list::rpush::RPushCommandPara::new(key.to_string(), values, Parameter::new()),
                        )))
                    }
                    "lpop" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let count = if let Some(count_str) = iter.next() {
                            let count_str = try_exact_bulk_string(Some(count_str))?;
                            Some(count_str.parse::<i64>().map_err(|_| anyhow::anyhow!("invalid count"))?)
                        } else {
                            None
                        };
                        Ok(CommandGroup::List(ListCommand::LPop(
                            crate::process::list::lpop::LPopCommandPara::new(key.to_string(), count, Parameter::new()),
                        )))
                    }
                    "rpop" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let count = if let Some(count_str) = iter.next() {
                            let count_str = try_exact_bulk_string(Some(count_str))?;
                            Some(count_str.parse::<i64>().map_err(|_| anyhow::anyhow!("invalid count"))?)
                        } else {
                            None
                        };
                        Ok(CommandGroup::List(ListCommand::RPop(
                            crate::process::list::rpop::RPopCommandPara::new(key.to_string(), count, Parameter::new()),
                        )))
                    }
                    "llen" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::List(ListCommand::LLen(
                            crate::process::list::llen::LLenCommandPara::new(key.to_string(), Parameter::new()),
                        )))
                    }
                    "lrange" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let start = try_exact_bulk_string(iter.next())?;
                        let stop = try_exact_bulk_string(iter.next())?;
                        
                        let start = start.parse::<i64>().map_err(|_| anyhow::anyhow!("invalid start index"))?;
                        let stop = stop.parse::<i64>().map_err(|_| anyhow::anyhow!("invalid stop index"))?;
                        
                        Ok(CommandGroup::List(ListCommand::LRange(
                            crate::process::list::lrange::LRangeCommandPara::new(key.to_string(), start, stop, Parameter::new()),
                        )))
                    }
                    
                    comm => {
                        error!("not support command: {}", comm);
                        Err(anyhow::anyhow!("not support command: {}", comm))
                    }
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
            info!("try_exact_bulk_string para: {:?}", para);
            Ok(para.val.as_str())
        }
        _ => Err(anyhow::anyhow!("invalid command")),
    }
}

// 手动实现Processor trait for CommandGroup
impl crate::Processor for CommandGroup {
    fn process(&self, data: &crate::Data) -> Result<Resp, anyhow::Error> {
        match self {
            CommandGroup::String(cmd) => cmd.process(data),
            CommandGroup::Hash(cmd) => cmd.process(data),
            CommandGroup::List(cmd) => cmd.process(data),
        }
    }
}

// 手动实现Processor trait for StringCommand
impl crate::Processor for StringCommand {
    fn process(&self, data: &crate::Data) -> Result<Resp, anyhow::Error> {
        match self {
            StringCommand::Set(cmd) => cmd.process(data),
            StringCommand::Get(cmd) => cmd.process(data),
            StringCommand::Del(cmd) => cmd.process(data),
            StringCommand::Exists(cmd) => cmd.process(data),
            StringCommand::Incr(cmd) => cmd.process(data),
            StringCommand::Decr(cmd) => cmd.process(data),
        }
    }
}

// 手动实现Processor trait for HashCommand
impl crate::Processor for HashCommand {
    fn process(&self, data: &crate::Data) -> Result<Resp, anyhow::Error> {
        match self {
            HashCommand::HSet(cmd) => cmd.process(data),
            HashCommand::HGet(cmd) => cmd.process(data),
            HashCommand::HDel(cmd) => cmd.process(data),
            HashCommand::HGetAll(cmd) => cmd.process(data),
            HashCommand::HKeys(cmd) => cmd.process(data),
            HashCommand::HVals(cmd) => cmd.process(data),
        }
    }
}

// 手动实现Processor trait for ListCommand
impl crate::Processor for ListCommand {
    fn process(&self, data: &crate::Data) -> Result<Resp, anyhow::Error> {
        match self {
            ListCommand::LPush(cmd) => cmd.process(data),
            ListCommand::RPush(cmd) => cmd.process(data),
            ListCommand::LPop(cmd) => cmd.process(data),
            ListCommand::RPop(cmd) => cmd.process(data),
            ListCommand::LLen(cmd) => cmd.process(data),
            ListCommand::LRange(cmd) => cmd.process(data),
        }
    }
}
