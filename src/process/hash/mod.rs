pub mod hset;
pub mod hget;
pub mod hdel;
pub mod hgetall;
pub mod hkeys;
pub mod hvals;

#[derive(Debug)]
pub enum HashCommand {
    HSet(hset::HSetCommandPara),
    HGet(hget::HGetCommandPara),
    HDel(hdel::HDelCommandPara),
    HGetAll(hgetall::HGetAllCommandPara),
    HKeys(hkeys::HKeysCommandPara),
    HVals(hvals::HValsCommandPara),
}
