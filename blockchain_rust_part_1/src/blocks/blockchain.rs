use tracing::info;

use crate::Block;

pub struct Blockchain {
    blocks: Vec<Block>,
    height: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::create_genesis_block()],
            height: 0,
        }
    }

    pub fn mine_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().unwrap();
        let block = Block::new(data, prev_block.get_hash().as_str());
        self.blocks.push(block);
        self.height += 1;
    }

    pub fn blocks_info(&self) {
        for block in self.blocks.iter() {
            info!("{:#?}", block);
        }
    }
}