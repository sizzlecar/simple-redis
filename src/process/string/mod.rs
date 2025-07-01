use crate::process::string::{get::GetCommandPara, set::SetCommandPara};

pub mod decr;
pub mod del;
pub mod exists;
pub mod expire;
pub mod get;
pub mod incr;
pub mod info;
pub mod keys;
pub mod mget;
pub mod mset;
pub mod persist;
pub mod scan;
pub mod set;
pub mod setex;
pub mod ttl;
pub mod type_cmd;

#[derive(Debug)]
pub enum StringCommand {
    Set(set::SetCommandPara),
    Get(get::GetCommandPara),
    Del(del::DelCommandPara),
    Exists(exists::ExistsCommandPara),
    Incr(incr::IncrCommandPara),
    Decr(decr::DecrCommandPara),
    Type(type_cmd::TypeCommandPara),
    Keys(keys::KeysCommandPara),
    Info(info::InfoCommandPara),
    Scan(scan::ScanCommandPara),
    Expire(expire::ExpireCommandPara),
    Ttl(ttl::TtlCommandPara),
    Persist(persist::PersistCommandPara),
    MGet(mget::MGetCommandPara),
    MSet(mset::MSetCommandPara),
    SetEx(setex::SetExCommandPara),
}
