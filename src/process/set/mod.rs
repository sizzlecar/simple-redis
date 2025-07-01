use crate::{Processor, Data, Resp};
use crate::process::Parameter;

pub mod sadd;
pub mod scard;
pub mod sdiff;
pub mod sinter;
pub mod sismember;
pub mod smembers;
pub mod smove;
pub mod spop;
pub mod srandmember;
pub mod srem;
pub mod sunion;

#[derive(Debug)]
pub enum SetCommand {
    SAdd(sadd::SAddCommandPara),
    SCard(scard::SCardCommandPara),
    SDiff(sdiff::SDiffCommandPara),
    SInter(sinter::SInterCommandPara),
    SIsMember(sismember::SIsMemberCommandPara),
    SMembers(smembers::SMembersCommandPara),
    SMove(smove::SMoveCommandPara),
    SPop(spop::SPopCommandPara),
    SRandMember(srandmember::SRandMemberCommandPara),
    SRem(srem::SRemCommandPara),
    SUnion(sunion::SUnionCommandPara),
}
