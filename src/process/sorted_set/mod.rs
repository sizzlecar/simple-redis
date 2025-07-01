use crate::process::Parameter;
use crate::{Data, Processor, Resp};

pub mod zadd;
pub mod zcard;
pub mod zcount;
pub mod zincrby;
pub mod zrange;
pub mod zrank;
pub mod zrem;
pub mod zremrangebyrank;
pub mod zremrangebyscore;
pub mod zrevrange;
pub mod zrevrank;
pub mod zscore;

#[derive(Debug)]
pub enum SortedSetCommand {
    ZAdd(zadd::ZAddCommandPara),
    ZCard(zcard::ZCardCommandPara),
    ZCount(zcount::ZCountCommandPara),
    ZIncrBy(zincrby::ZIncrByCommandPara),
    ZRange(zrange::ZRangeCommandPara),
    ZRank(zrank::ZRankCommandPara),
    ZRem(zrem::ZRemCommandPara),
    ZRemRangeByRank(zremrangebyrank::ZRemRangeByRankCommandPara),
    ZRemRangeByScore(zremrangebyscore::ZRemRangeByScoreCommandPara),
    ZRevRange(zrevrange::ZRevRangeCommandPara),
    ZRevRank(zrevrank::ZRevRankCommandPara),
    ZScore(zscore::ZScoreCommandPara),
}
