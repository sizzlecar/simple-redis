pub mod llen;
pub mod lpop;
pub mod lpush;
pub mod lrange;
pub mod lrem;
pub mod rpop;
pub mod rpush;

#[derive(Debug)]
pub enum ListCommand {
    LPush(lpush::LPushCommandPara),
    RPush(rpush::RPushCommandPara),
    LPop(lpop::LPopCommandPara),
    RPop(rpop::RPopCommandPara),
    LLen(llen::LLenCommandPara),
    LRange(lrange::LRangeCommandPara),
    LRem(lrem::LRemCommandPara),
}
