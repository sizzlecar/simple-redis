pub mod hdel;
pub mod hget;
pub mod hgetall;
pub mod hkeys;
pub mod hmget;
pub mod hmset;
pub mod hset;
pub mod hvals;

#[derive(Debug)]
pub enum HashCommand {
    HSet(hset::HSetCommandPara),
    HGet(hget::HGetCommandPara),
    HDel(hdel::HDelCommandPara),
    HGetAll(hgetall::HGetAllCommandPara),
    HKeys(hkeys::HKeysCommandPara),
    HVals(hvals::HValsCommandPara),
    HMGet(hmget::HMGetCommandPara),
    HMSet(hmset::HMSetCommandPara),
}
