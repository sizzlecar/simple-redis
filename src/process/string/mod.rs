use enum_dispatch::enum_dispatch;

use crate::process::string::{get::GetCommandPara, set::SetCommandPara};

pub mod get;
pub mod set;

#[derive(Debug)]
#[enum_dispatch(Processor)]
pub enum StringCommand {
    Set(SetCommandPara),
    Get(GetCommandPara),
}
