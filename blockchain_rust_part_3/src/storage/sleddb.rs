use std::path::Path;
use sled::{Db, IVec, transaction::TransactionResult};

use crate::{Storage, error::BlockchainError, Block, utils::{deserialize, serialize}, TIP_KEY, TABLE_OF_BLOCK, HEIGHT, StorageIterator};

pub struct SledDb {
    db: Db
}

impl SledDb {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            db: sled::open(path).unwrap()
        }
    }

    fn get_full_key(table: &str, key: &str) -> String {
        format!("{}:{}", table, key)
    }
}

impl Storage for SledDb {
    fn get_tip(&self) -> Result<Option<String>, BlockchainError> {
        let result = self.db.get(TIP_KEY)?.map(|v| deserialize::<String>(&v.to_vec()));
        result.map_or(Ok(None), |v| v.map(Some))
    }

    fn get_block(&self, key: &str) -> Result<Option<Block>, BlockchainError> {
        let name = Self::get_full_key(TABLE_OF_BLOCK, key);
        let result = self.db.get(name)?.map(|v| v.into());
        Ok(result)
    }

    fn get_height(&self) -> Result<Option<usize>, BlockchainError> {
        let result = self.db.get(HEIGHT)?.map(|v| deserialize::<usize>(&v.to_vec()));
        result.map_or(Ok(None), |v| v.map(Some))
    }

    fn update_blocks(&self, key: &str, block: &Block, height: usize) {
        let _: TransactionResult<(), ()> = self.db.transaction(|db| {
            let name = Self::get_full_key(TABLE_OF_BLOCK, key);
            db.insert(name.as_str(), serialize(block).unwrap())?;
            db.insert(TIP_KEY, serialize(key).unwrap())?;
            db.insert(HEIGHT, serialize(&height).unwrap())?;
            db.flush();
            Ok(())
        });
    }

    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>, BlockchainError> {
        let prefix = format!("{}:", TABLE_OF_BLOCK);
        let iter = StorageIterator::new(self.db.scan_prefix(prefix));
        Ok(Box::new(iter))
    }
}

impl From<IVec> for Block {
    fn from(v: IVec) -> Self {
        let result = deserialize::<Block>(&v.to_vec());
        match result {
            Ok(block) => block,
            Err(_) => Block::default(),
        }
    }
}

impl From<Result<(IVec, IVec), sled::Error>> for Block {
    fn from(result: Result<(IVec, IVec), sled::Error>) -> Self {
        match result {
            Ok((_, v)) => match deserialize::<Block>(&v.to_vec()) {
                    Ok(block) => block,
                    Err(_) => Block::default(),
            },
            Err(_) => Block::default(),
        }
    }
}