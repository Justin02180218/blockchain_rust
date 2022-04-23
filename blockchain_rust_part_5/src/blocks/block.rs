use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::{ProofOfWork, Transaction, utils::{serialize, hash_to_str}};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct BlockHeader {
    timestamp: i64,
    prev_hash: String,
    txs_hash: String,
    bits: usize,
    nonce: usize,
}

impl BlockHeader {
    fn new(prev_hash: &str, bits: usize) -> Self {
        Self {
            timestamp: Utc::now().timestamp(),
            prev_hash: prev_hash.into(),
            txs_hash: String::new(),
            bits,
            nonce: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Block {
    header: BlockHeader,
    tranxs: Vec<Transaction>,
    hash: String,
}

impl Block {
    pub fn new(txs: &[Transaction], prev_hash: &str, bits: usize) -> Self {
        let mut block = Block {
            header: BlockHeader::new(prev_hash, bits),
            tranxs: txs.to_vec(),
            hash: String::new(),
        };
        block.set_txs_hash(txs);

        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }

    pub fn create_genesis_block(bits: usize, genesis_addr: &str) -> Self {
        let coinbase = Transaction::new_coinbase(genesis_addr);
        Self::new(&vec![coinbase], "", bits)
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_header(&self) -> BlockHeader {
        self.header.clone()
    }

    pub fn set_nonce(&mut self, nonce: usize) {
        self.header.nonce = nonce;
    }

    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }

    fn set_txs_hash(&mut self, txs: &[Transaction]) {
        if let Ok(txs_ser) = serialize(txs) {
            self.header.txs_hash = hash_to_str(&txs_ser);
        }
    }

    pub fn get_tranxs(&self) -> Vec<Transaction> {
        self.tranxs.clone()
    }
}
