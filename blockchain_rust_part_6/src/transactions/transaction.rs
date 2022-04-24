use serde::{Serialize, Deserialize};

use crate::{Txinput, Txoutput, utils::{serialize, hash_to_str, ecdsa_p256_sha256_sign_digest, ecdsa_p256_sha256_sign_verify}, UTXOSet, Storage, Wallets, hash_pub_key, Blockchain};

const SUBSIDY: i32= 10;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Transaction {
    id: String,
    vin: Vec<Txinput>,
    vout: Vec<Txoutput>,
}

impl Transaction {
    pub fn new_coinbase(to: &str) -> Self {
        let txin = Txinput::default();
        let txout = Txoutput::new(SUBSIDY, to);
        
        let mut tx = Transaction {
            id: String::new(),
            vin: vec![txin],
            vout: vec![txout],
        };
        tx.set_hash();

        tx
    }

    pub fn new_utxo<T: Storage>(from: &str, to: &str, amount: i32, utxo_set: &UTXOSet<T>, bc: &Blockchain<T>) -> Self {
        let wallets = Wallets::new().unwrap();
        let wallet = wallets.get_wallet(from).unwrap();
        let public_key_hash = hash_pub_key(wallet.get_public_key());
        
        let (accumulated, valid_outputs) = utxo_set.find_spendable_outputs(&public_key_hash, amount);
        if accumulated < amount {
            panic!("Error not enough funds");
        }

        let mut inputs = vec![];
        for (txid, outputs) in valid_outputs {
            for idx in outputs {
                let input = Txinput::new(txid.clone(), idx.clone(), wallet.get_public_key().to_vec());
                inputs.push(input);
            }
        }

        let mut outputs = vec![Txoutput::new(amount, &to)];
        if accumulated > amount {
            outputs.push(Txoutput::new(accumulated - amount, &from));
        }

        let mut tx = Transaction {
            id: String::new(),
            vin: inputs,
            vout: outputs,
        };
        tx.set_hash();
        tx.sign(bc, wallet.get_pkcs8());
        
        tx
    }

    fn set_hash(&mut self) {
        if let Ok(tx_ser) = serialize(self) {
            self.id = hash_to_str(&tx_ser)
        }
    }

    fn sign<T: Storage>(&mut self, bc: &Blockchain<T>, pkcs8: &[u8]) {
        let mut tx_copy = self.trimmed_copy();

        for (idx, vin) in self.vin.iter_mut().enumerate() {
            // 查找输入引用的交易
            let prev_tx_option = bc.find_transaction(vin.get_txid());
            if prev_tx_option.is_none() {
                panic!("ERROR: Previous transaction is not correct")
            }
            let prev_tx = prev_tx_option.unwrap();
            tx_copy.vin[idx].set_signature(vec![]);
            tx_copy.vin[idx].set_pub_key(prev_tx.vout[vin.get_vout()].get_pub_key_hash());
            tx_copy.set_hash();

            tx_copy.vin[idx].set_pub_key(&vec![]);

            // 使用私钥对数据签名
            let signature = ecdsa_p256_sha256_sign_digest(pkcs8, tx_copy.id.as_bytes());
            vin.set_signature(signature);
        }
    }

    pub fn verify<T: Storage>(&self, bc: &Blockchain<T>) -> bool {
        if self.is_coinbase() {
            return true;
        }
        let mut tx_copy = self.trimmed_copy();
        for (idx, vin) in self.vin.iter().enumerate() {
            let prev_tx_option = bc.find_transaction(vin.get_txid());
            if prev_tx_option.is_none() {
                panic!("ERROR: Previous transaction is not correct")
            }
            let prev_tx = prev_tx_option.unwrap();
            tx_copy.vin[idx].set_signature(vec![]);
            tx_copy.vin[idx].set_pub_key(prev_tx.vout[vin.get_vout()].get_pub_key_hash());
            tx_copy.set_hash();

            tx_copy.vin[idx].set_pub_key(&vec![]);

            // 使用公钥验证签名
            let verify = ecdsa_p256_sha256_sign_verify(
                vin.get_pub_key(),
                vin.get_signature(),
                tx_copy.id.as_bytes(),
            );
            if !verify {
                return false;
            }
        }
        true
    }

    /// 判断是否是 coinbase 交易
    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].get_pub_key().len() == 0
    }

    fn trimmed_copy(&self) -> Transaction {
        let mut inputs = vec![];
        let mut outputs = vec![];
        for input in &self.vin {
            let txinput = Txinput::new(input.get_txid(), input.get_vout(), vec![]);
            inputs.push(txinput);
        }
        for output in &self.vout {
            outputs.push(output.clone());
        }
        Transaction {
            id: self.id.clone(),
            vin: inputs,
            vout: outputs,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_vout(&self) -> &[Txoutput] {
        self.vout.as_slice()
    }

    pub fn get_vin(&self) -> &[Txinput] {
        self.vin.as_slice()
    }
}


