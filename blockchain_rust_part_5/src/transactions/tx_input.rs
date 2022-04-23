use serde::{Serialize, Deserialize};

use crate::hash_pub_key;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Txinput {
    txid: String,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>
}

impl Txinput {
    pub fn new(txid: String, vout: usize, pub_key: Vec<u8>) -> Self {
        Self {
            txid,
            vout,
            signature: vec![],
            pub_key,
        }
    }

    pub fn can_unlock_output(&self, pub_key_hash: &[u8]) -> bool {
        let locked_hash = hash_pub_key(&self.pub_key);
        locked_hash.eq(pub_key_hash)
    }

    pub fn get_txid(&self) -> String {
        self.txid.clone()
    }

    pub fn get_vout(&self) -> usize {
        self.vout
    }

    pub fn get_pub_key(&self) -> &[u8] {
        self.pub_key.as_slice()
    }

    pub fn get_signature(&self) -> &[u8] {
        self.signature.as_slice()
    }

    pub fn set_signature(&mut self, signature: Vec<u8>) {
        self.signature = signature
    }

    pub fn set_pub_key(&mut self, pub_key: &[u8]) {
        self.pub_key = pub_key.to_vec();
    }
}