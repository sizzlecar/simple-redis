use tracing::{debug, info};

use crate::process::hash::HashCommand;
use crate::process::list::ListCommand;
use crate::process::set::SetCommand;
use crate::process::sorted_set::SortedSetCommand;
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
pub enum CommandGroup {
    String(StringCommand),
    Hash(HashCommand),
    List(ListCommand),
    Set(SetCommand),
    SortedSet(SortedSetCommand),
}

impl TryFrom<Resp> for CommandGroup {
    type Error = anyhow::Error;

    fn try_from(value: Resp) -> Result<Self, Self::Error> {
        debug!("Parsing command from RESP array");
        match value {
            Resp::Arrays(arr) => {
                let mut iter = arr.val.iter();
                let command = try_exact_bulk_string(iter.next())?;
                info!("🎯 Executing command: {}", &command.to_uppercase());
                match command.to_lowercase().as_str() {
                    // String commands
                    "set" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let value = try_exact_bulk_string(iter.next())?;
                        info!("💾 SET operation: key='{}', value='{}'", key, value);
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
                        info!("🔍 GET operation: key='{}'", key);
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
                            crate::process::string::del::DelCommandPara::new(
                                keys,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "exists" => {
                        let mut keys = Vec::new();
                        for item in iter {
                            let key = try_exact_bulk_string(Some(item))?;
                            keys.push(key.to_string());
                        }
                        Ok(CommandGroup::String(StringCommand::Exists(
                            crate::process::string::exists::ExistsCommandPara::new(
                                keys,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "incr" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::String(StringCommand::Incr(
                            crate::process::string::incr::IncrCommandPara::new(
                                key.to_string(),
                                None,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "decr" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::String(StringCommand::Decr(
                            crate::process::string::decr::DecrCommandPara::new(
                                key.to_string(),
                                None,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "type" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        info!("🔍 TYPE operation: key='{}'", key);
                        Ok(CommandGroup::String(StringCommand::Type(
                            crate::process::string::type_cmd::TypeCommandPara::new(
                                key.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "keys" => {
                        let pattern = try_exact_bulk_string(iter.next())?;
                        info!("🔑 KEYS operation: pattern='{}'", pattern);
                        Ok(CommandGroup::String(StringCommand::Keys(
                            crate::process::string::keys::KeysCommandPara::new(
                                pattern.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "expire" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let seconds = try_exact_bulk_string(iter.next())?;
                        let seconds = seconds
                            .parse::<u64>()
                            .map_err(|_| anyhow::anyhow!("invalid expire time"))?;
                        info!("⏰ EXPIRE operation: key='{}', seconds={}", key, seconds);
                        Ok(CommandGroup::String(StringCommand::Expire(
                            crate::process::string::expire::ExpireCommandPara::new(
                                key.to_string(),
                                seconds,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "ttl" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        info!("⏰ TTL operation: key='{}'", key);
                        Ok(CommandGroup::String(StringCommand::Ttl(
                            crate::process::string::ttl::TtlCommandPara::new(
                                key.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "persist" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        info!("⏰ PERSIST operation: key='{}'", key);
                        Ok(CommandGroup::String(StringCommand::Persist(
                            crate::process::string::persist::PersistCommandPara::new(
                                key.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "mget" => {
                        let mut keys = Vec::new();
                        for item in iter {
                            let key = try_exact_bulk_string(Some(item))?;
                            keys.push(key.to_string());
                        }
                        info!("🔍 MGET operation: {} keys", keys.len());
                        Ok(CommandGroup::String(StringCommand::MGet(
                            crate::process::string::mget::MGetCommandPara::new(
                                keys,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "mset" => {
                        let args: Vec<&str> = iter
                            .map(|item| try_exact_bulk_string(Some(item)))
                            .collect::<Result<Vec<_>, _>>()?;

                        if args.len() % 2 != 0 {
                            return Err(anyhow::anyhow!("wrong number of arguments for MSET"));
                        }

                        let mut key_values = Vec::new();
                        for chunk in args.chunks(2) {
                            key_values.push((chunk[0].to_string(), chunk[1].to_string()));
                        }

                        info!("💾 MSET operation: {} key-value pairs", key_values.len());
                        Ok(CommandGroup::String(StringCommand::MSet(
                            crate::process::string::mset::MSetCommandPara::new(
                                key_values,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "setex" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let seconds = try_exact_bulk_string(iter.next())?;
                        let value = try_exact_bulk_string(iter.next())?;
                        
                        let seconds = seconds
                            .parse::<u64>()
                            .map_err(|_| anyhow::anyhow!("invalid expire time"))?;

                        info!("💾⏰ SETEX operation: key='{}', seconds={}, value='{}'", key, seconds, value);
                        Ok(CommandGroup::String(StringCommand::SetEx(
                            crate::process::string::setex::SetExCommandPara::new(
                                key.to_string(),
                                seconds,
                                value.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }

                    // Hash commands
                    "hset" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let mut field_values = Vec::new();

                        let fields_and_values: Vec<&str> = iter
                            .map(|item| try_exact_bulk_string(Some(item)))
                            .collect::<Result<Vec<_>, _>>()?;

                        if fields_and_values.len() % 2 != 0 {
                            return Err(anyhow::anyhow!("wrong number of arguments for HSET"));
                        }

                        for chunk in fields_and_values.chunks(2) {
                            field_values.push((chunk[0].to_string(), chunk[1].to_string()));
                        }

                        info!(
                            "🗂️ HSET operation: key='{}', {} field-value pairs",
                            key,
                            field_values.len()
                        );

                        Ok(CommandGroup::Hash(HashCommand::HSet(
                            crate::process::hash::hset::HSetCommandPara::new(
                                key.to_string(),
                                field_values,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "hget" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let field = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::Hash(HashCommand::HGet(
                            crate::process::hash::hget::HGetCommandPara::new(
                                key.to_string(),
                                field.to_string(),
                                Parameter::new(),
                            ),
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
                            crate::process::hash::hdel::HDelCommandPara::new(
                                key.to_string(),
                                fields,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "hgetall" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::Hash(HashCommand::HGetAll(
                            crate::process::hash::hgetall::HGetAllCommandPara::new(
                                key.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "hkeys" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::Hash(HashCommand::HKeys(
                            crate::process::hash::hkeys::HKeysCommandPara::new(
                                key.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "hvals" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::Hash(HashCommand::HVals(
                            crate::process::hash::hvals::HValsCommandPara::new(
                                key.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "hmget" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let mut fields = Vec::new();
                        for item in iter {
                            let field = try_exact_bulk_string(Some(item))?;
                            fields.push(field.to_string());
                        }
                        info!("🗂️ HMGET operation: key='{}', {} fields", key, fields.len());
                        Ok(CommandGroup::Hash(HashCommand::HMGet(
                            crate::process::hash::hmget::HMGetCommandPara::new(
                                key.to_string(),
                                fields,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "hmset" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let args: Vec<&str> = iter
                            .map(|item| try_exact_bulk_string(Some(item)))
                            .collect::<Result<Vec<_>, _>>()?;

                        if args.len() % 2 != 0 {
                            return Err(anyhow::anyhow!("wrong number of arguments for HMSET"));
                        }

                        let mut field_values = Vec::new();
                        for chunk in args.chunks(2) {
                            field_values.push((chunk[0].to_string(), chunk[1].to_string()));
                        }

                        info!("🗂️ HMSET operation: key='{}', {} field-value pairs", key, field_values.len());
                        Ok(CommandGroup::Hash(HashCommand::HMSet(
                            crate::process::hash::hmset::HMSetCommandPara::new(
                                key.to_string(),
                                field_values,
                                Parameter::new(),
                            ),
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
                        info!("📋 LPUSH operation: key='{}', {} values", key, values.len());
                        Ok(CommandGroup::List(ListCommand::LPush(
                            crate::process::list::lpush::LPushCommandPara::new(
                                key.to_string(),
                                values,
                                Parameter::new(),
                            ),
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
                            crate::process::list::rpush::RPushCommandPara::new(
                                key.to_string(),
                                values,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "lpop" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let count = if let Some(count_str) = iter.next() {
                            let count_str = try_exact_bulk_string(Some(count_str))?;
                            Some(
                                count_str
                                    .parse::<i64>()
                                    .map_err(|_| anyhow::anyhow!("invalid count"))?,
                            )
                        } else {
                            None
                        };
                        Ok(CommandGroup::List(ListCommand::LPop(
                            crate::process::list::lpop::LPopCommandPara::new(
                                key.to_string(),
                                count,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "rpop" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let count = if let Some(count_str) = iter.next() {
                            let count_str = try_exact_bulk_string(Some(count_str))?;
                            Some(
                                count_str
                                    .parse::<i64>()
                                    .map_err(|_| anyhow::anyhow!("invalid count"))?,
                            )
                        } else {
                            None
                        };
                        Ok(CommandGroup::List(ListCommand::RPop(
                            crate::process::list::rpop::RPopCommandPara::new(
                                key.to_string(),
                                count,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "llen" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::List(ListCommand::LLen(
                            crate::process::list::llen::LLenCommandPara::new(
                                key.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "lrange" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let start = try_exact_bulk_string(iter.next())?;
                        let stop = try_exact_bulk_string(iter.next())?;

                        let start = start
                            .parse::<i64>()
                            .map_err(|_| anyhow::anyhow!("invalid start index"))?;
                        let stop = stop
                            .parse::<i64>()
                            .map_err(|_| anyhow::anyhow!("invalid stop index"))?;

                        Ok(CommandGroup::List(ListCommand::LRange(
                            crate::process::list::lrange::LRangeCommandPara::new(
                                key.to_string(),
                                start,
                                stop,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "lrem" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let count = try_exact_bulk_string(iter.next())?;
                        let element = try_exact_bulk_string(iter.next())?;

                        let count = count
                            .parse::<i64>()
                            .map_err(|_| anyhow::anyhow!("invalid count"))?;

                        info!(
                            "🗑️ LREM operation: key='{}', count={}, element='{}'",
                            key, count, element
                        );
                        Ok(CommandGroup::List(ListCommand::LRem(
                            crate::process::list::lrem::LRemCommandPara::new(
                                key.to_string(),
                                count,
                                element.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }

                    // Set commands
                    "sadd" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let mut members = Vec::new();
                        for item in iter {
                            let member = try_exact_bulk_string(Some(item))?;
                            members.push(member.to_string());
                        }
                        info!(
                            "🔷 SADD operation: key='{}', {} members",
                            key,
                            members.len()
                        );
                        Ok(CommandGroup::Set(SetCommand::SAdd(
                            crate::process::set::sadd::SAddCommandPara::new(
                                key.to_string(),
                                members,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "scard" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::Set(SetCommand::SCard(
                            crate::process::set::scard::SCardCommandPara::new(
                                key.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "smembers" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::Set(SetCommand::SMembers(
                            crate::process::set::smembers::SMembersCommandPara::new(
                                key.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "srem" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let mut members = Vec::new();
                        for item in iter {
                            let member = try_exact_bulk_string(Some(item))?;
                            members.push(member.to_string());
                        }
                        Ok(CommandGroup::Set(SetCommand::SRem(
                            crate::process::set::srem::SRemCommandPara::new(
                                key.to_string(),
                                members,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "sismember" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let member = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::Set(SetCommand::SIsMember(
                            crate::process::set::sismember::SIsMemberCommandPara::new(
                                key.to_string(),
                                member.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }

                    // Sorted Set commands
                    "zadd" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let mut score_members = Vec::new();

                        let args: Vec<&str> = iter
                            .map(|item| try_exact_bulk_string(Some(item)))
                            .collect::<Result<Vec<_>, _>>()?;

                        if args.len() % 2 != 0 {
                            return Err(anyhow::anyhow!("wrong number of arguments for ZADD"));
                        }

                        for chunk in args.chunks(2) {
                            let score = chunk[0]
                                .parse::<f64>()
                                .map_err(|_| anyhow::anyhow!("invalid score"))?;
                            let member = chunk[1].to_string();
                            score_members.push((score, member));
                        }

                        info!(
                            "📊 ZADD operation: key='{}', {} score-member pairs",
                            key,
                            score_members.len()
                        );
                        Ok(CommandGroup::SortedSet(SortedSetCommand::ZAdd(
                            crate::process::sorted_set::zadd::ZAddCommandPara::new(
                                key.to_string(),
                                score_members,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "zcard" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::SortedSet(SortedSetCommand::ZCard(
                            crate::process::sorted_set::zcard::ZCardCommandPara::new(
                                key.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "zscore" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let member = try_exact_bulk_string(iter.next())?;
                        Ok(CommandGroup::SortedSet(SortedSetCommand::ZScore(
                            crate::process::sorted_set::zscore::ZScoreCommandPara::new(
                                key.to_string(),
                                member.to_string(),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "zrem" => {
                        let key = try_exact_bulk_string(iter.next())?;
                        let mut members = Vec::new();
                        for item in iter {
                            let member = try_exact_bulk_string(Some(item))?;
                            members.push(member.to_string());
                        }
                        Ok(CommandGroup::SortedSet(SortedSetCommand::ZRem(
                            crate::process::sorted_set::zrem::ZRemCommandPara::new(
                                key.to_string(),
                                members,
                                Parameter::new(),
                            ),
                        )))
                    }

                    // Management commands (Redis客户端常用的管理命令)
                    "ping" => {
                        // PING命令，返回PONG
                        debug!("PING command received");
                        Ok(CommandGroup::String(StringCommand::Get(
                            GetCommandPara::new(
                                Some("__ping__".to_string()),
                                None,
                                Parameter::new(),
                            ),
                        )))
                    }

                    "client" => {
                        // CLIENT命令 (如CLIENT SETNAME)，简单返回OK
                        debug!("CLIENT command received, returning OK");
                        Ok(CommandGroup::String(StringCommand::Set(
                            SetCommandPara::new(
                                Some("__client__".to_string()),
                                Some("OK".to_string()),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "info" => {
                        // INFO命令，返回真实的服务器信息
                        let section = iter
                            .next()
                            .map(|s| try_exact_bulk_string(Some(s)).unwrap_or_default());
                        info!("ℹ️ INFO operation: section={:?}", section);
                        Ok(CommandGroup::String(StringCommand::Info(
                            crate::process::string::info::InfoCommandPara::new(
                                section.map(|s| s.to_string()),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "scan" => {
                        // SCAN命令，用于遍历键
                        let cursor_str = try_exact_bulk_string(iter.next())?;
                        let cursor = cursor_str
                            .parse::<u64>()
                            .map_err(|_| anyhow::anyhow!("invalid cursor"))?;

                        let mut pattern = None;
                        let mut count = None;

                        // 解析可选参数
                        while let Some(arg) = iter.next() {
                            let arg_str = try_exact_bulk_string(Some(arg))?;
                            match arg_str.to_uppercase().as_str() {
                                "MATCH" => {
                                    if let Some(pattern_arg) = iter.next() {
                                        pattern = Some(
                                            try_exact_bulk_string(Some(pattern_arg))?.to_string(),
                                        );
                                    }
                                }
                                "COUNT" => {
                                    if let Some(count_arg) = iter.next() {
                                        let count_str = try_exact_bulk_string(Some(count_arg))?;
                                        count = Some(
                                            count_str
                                                .parse::<u64>()
                                                .map_err(|_| anyhow::anyhow!("invalid count"))?,
                                        );
                                    }
                                }
                                _ => {
                                    // 忽略未知参数
                                }
                            }
                        }

                        info!(
                            "🔍 SCAN operation: cursor={}, pattern={:?}, count={:?}",
                            cursor, pattern, count
                        );
                        Ok(CommandGroup::String(StringCommand::Scan(
                            crate::process::string::scan::ScanCommandPara::new(
                                cursor,
                                pattern,
                                count,
                                Parameter::new(),
                            ),
                        )))
                    }
                    "select" => {
                        // SELECT命令，选择数据库，简单返回OK
                        debug!("SELECT command received, returning OK");
                        Ok(CommandGroup::String(StringCommand::Set(
                            SetCommandPara::new(
                                Some("__select__".to_string()),
                                Some("OK".to_string()),
                                Parameter::new(),
                            ),
                        )))
                    }
                    "command" => {
                        // COMMAND命令，返回空数组
                        debug!("COMMAND command received");
                        Ok(CommandGroup::String(StringCommand::Get(
                            GetCommandPara::new(
                                Some("__command__".to_string()),
                                None,
                                Parameter::new(),
                            ),
                        )))
                    }

                    comm => {
                        debug!("Unsupported command: {}, returning generic OK", comm);
                        // 对于不支持的命令，返回一个友好的错误而不是panic
                        Ok(CommandGroup::String(StringCommand::Set(
                            SetCommandPara::new(
                                Some("__unsupported__".to_string()),
                                Some("OK".to_string()),
                                Parameter::new(),
                            ),
                        )))
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
            CommandGroup::Set(cmd) => cmd.process(data),
            CommandGroup::SortedSet(cmd) => cmd.process(data),
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
            StringCommand::Type(cmd) => cmd.process(data),
            StringCommand::Keys(cmd) => cmd.process(data),
            StringCommand::Info(cmd) => cmd.process(data),
            StringCommand::Scan(cmd) => cmd.process(data),
            StringCommand::Expire(cmd) => cmd.process(data),
            StringCommand::Ttl(cmd) => cmd.process(data),
            StringCommand::Persist(cmd) => cmd.process(data),
            StringCommand::MGet(cmd) => cmd.process(data),
            StringCommand::MSet(cmd) => cmd.process(data),
            StringCommand::SetEx(cmd) => cmd.process(data),
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
            HashCommand::HMGet(cmd) => cmd.process(data),
            HashCommand::HMSet(cmd) => cmd.process(data),
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
            ListCommand::LRem(cmd) => cmd.process(data),
        }
    }
}

// 手动实现Processor trait for SetCommand
impl crate::Processor for SetCommand {
    fn process(&self, data: &crate::Data) -> Result<Resp, anyhow::Error> {
        match self {
            SetCommand::SAdd(cmd) => cmd.process(data),
            SetCommand::SCard(cmd) => cmd.process(data),
            SetCommand::SDiff(cmd) => cmd.process(data),
            SetCommand::SInter(cmd) => cmd.process(data),
            SetCommand::SIsMember(cmd) => cmd.process(data),
            SetCommand::SMembers(cmd) => cmd.process(data),
            SetCommand::SMove(cmd) => cmd.process(data),
            SetCommand::SPop(cmd) => cmd.process(data),
            SetCommand::SRandMember(cmd) => cmd.process(data),
            SetCommand::SRem(cmd) => cmd.process(data),
            SetCommand::SUnion(cmd) => cmd.process(data),
        }
    }
}

// 手动实现Processor trait for SortedSetCommand
impl crate::Processor for SortedSetCommand {
    fn process(&self, data: &crate::Data) -> Result<Resp, anyhow::Error> {
        match self {
            SortedSetCommand::ZAdd(cmd) => cmd.process(data),
            SortedSetCommand::ZCard(cmd) => cmd.process(data),
            SortedSetCommand::ZCount(cmd) => cmd.process(data),
            SortedSetCommand::ZIncrBy(cmd) => cmd.process(data),
            SortedSetCommand::ZRange(cmd) => cmd.process(data),
            SortedSetCommand::ZRank(cmd) => cmd.process(data),
            SortedSetCommand::ZRem(cmd) => cmd.process(data),
            SortedSetCommand::ZRemRangeByRank(cmd) => cmd.process(data),
            SortedSetCommand::ZRemRangeByScore(cmd) => cmd.process(data),
            SortedSetCommand::ZRevRange(cmd) => cmd.process(data),
            SortedSetCommand::ZRevRank(cmd) => cmd.process(data),
            SortedSetCommand::ZScore(cmd) => cmd.process(data),
        }
    }
}
