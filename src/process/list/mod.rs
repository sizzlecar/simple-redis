pub mod lpush;
pub mod rpush;
pub mod lpop;
pub mod rpop;
pub mod llen;
pub mod lrange;

#[derive(Debug)]
pub enum ListCommand {
    LPush(lpush::LPushCommandPara),
    RPush(rpush::RPushCommandPara),
    LPop(lpop::LPopCommandPara),
    RPop(rpop::RPopCommandPara),
    LLen(llen::LLenCommandPara),
    LRange(lrange::LRangeCommandPara),
}
