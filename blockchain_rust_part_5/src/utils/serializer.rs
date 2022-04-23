use anyhow::Result;
use crypto::{sha3::Sha3, digest::Digest};
use serde::{Serialize, Deserialize};

use crate::error::BlockchainError;

pub fn serialize<T>(data: &T) -> Result<Vec<u8>, BlockchainError> 
where
    T: Serialize + ?Sized
{
    Ok(bincode::serialize(data)?)
}

#[allow(dead_code)]
pub fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, BlockchainError> 
where
    T: Deserialize<'a> + ?Sized
{
    Ok(bincode::deserialize(data)?)
}

pub fn hash_to_str(data: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result_str()
}

pub fn hash_to_u8(data: &[u8], out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result(out);
}