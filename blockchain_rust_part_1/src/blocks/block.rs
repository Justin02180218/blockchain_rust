use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::utils::{serialize, hash_to_str};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockHeader {
    timestamp: i64,
    prev_hash: String,
    nonce: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    header: BlockHeader,
    data: String,
    hash: String,
}

impl Block {
    pub fn new(data: &str, prev_hash: &str) -> Self {
        let mut block = Block {
            header: BlockHeader { 
                timestamp: Utc::now().timestamp(), 
                prev_hash: prev_hash.into(), 
                nonce: 0, 
            },
            data: data.into(),
            hash: String::new(),
        };
        block.set_hash();

        block
    }

    pub fn create_genesis_block() -> Self {
        Self::new("创世区块", "")
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    fn set_hash(&mut self) {
        if let Ok(serialized) = serialize(&self.header) {
            self.hash = hash_to_str(&serialized)
        }
    }
}
