use crate::process::string::{get::GetCommandPara, set::SetCommandPara};

pub mod get;
pub mod set;
pub mod del;
pub mod exists;
pub mod incr;
pub mod decr;

#[derive(Debug)]
pub enum StringCommand {
    Set(SetCommandPara),
    Get(GetCommandPara),
    Del(del::DelCommandPara),
    Exists(exists::ExistsCommandPara),
    Incr(incr::IncrCommandPara),
    Decr(decr::DecrCommandPara),
}
