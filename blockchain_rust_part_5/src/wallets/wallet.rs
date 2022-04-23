use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
use serde::{Serialize, Deserialize};
use crate::utils::{new_private_key, base58_encode, sha256_digest, ripemd160_digest};

const VERSION: u8 = 0x00;
pub const ADDRESS_CHECKSUM_LEN: usize = 4;

#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    pkcs8: Vec<u8>,
    public_key: Vec<u8>,
}

impl Wallet {
    pub fn new() -> Self {
        let pkcs8 = new_private_key();
        let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref()).unwrap();
        let public_key = key_pair.public_key().as_ref().to_vec();

        Self { pkcs8, public_key }
    }

    pub fn get_address(&self) -> String {
        let pub_key_hash = hash_pub_key(self.public_key.as_slice());
        let mut payload = vec![];
        payload.push(VERSION);
        payload.extend(pub_key_hash.as_slice());
        let checksum = checksum(payload.as_slice());
        payload.extend(checksum.as_slice());
        base58_encode(payload.as_slice())
    }

    pub fn get_pkcs8(&self) -> &[u8] {
        self.pkcs8.as_slice()
    }

    pub fn get_public_key(&self) -> &[u8] {
        self.public_key.as_slice()
    }
}

pub fn hash_pub_key(pub_key: &[u8]) -> Vec<u8> {
    let pub_key_sha256 = sha256_digest(pub_key);
    let pub_key_ripemd160 = ripemd160_digest(&pub_key_sha256);
    pub_key_ripemd160
}

pub fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = sha256_digest(payload);
    let second_sha = sha256_digest(&first_sha);
    second_sha[0..ADDRESS_CHECKSUM_LEN].to_vec()
}