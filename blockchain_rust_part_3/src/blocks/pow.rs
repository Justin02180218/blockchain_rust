use anyhow::Result;
use std::ops::Shl;
use bigint::U256;
use crate::{utils::{serialize, hash_to_u8, hash_to_str}, Block};

const MAX_NONCE: usize = usize::MAX;

pub struct ProofOfWork {
    target: U256,
}

impl ProofOfWork {
    pub fn new(bits: usize) -> Self {
        let mut target = U256::from(1 as usize);
        target = target.shl(256 - bits);

        Self {
            target
        }
    }

    pub fn run(&self, block: &mut Block) {
        let mut nonce = 0;
        while nonce < MAX_NONCE {
            if let Ok(pre_hash) = Self::prepare_data(block, nonce) {
                let mut hash_u: [u8; 32] = [0; 32];
                hash_to_u8(&pre_hash,&mut hash_u);
                let pre_hash_int = U256::from(hash_u);

                if pre_hash_int.lt(&(self.target)) {
                    block.set_hash(hash_to_str(&pre_hash));
                    break;
                }else {
                    nonce += 1;
                }
            }
        }
    }

    fn prepare_data(block: &mut Block, nonce: usize) -> Result<Vec<u8>> {
        block.set_nonce(nonce);
        Ok(serialize(&(block.get_header()))?)
    }
}