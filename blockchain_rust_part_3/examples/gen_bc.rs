use std::env::current_dir;

use blockchain_rust_part_3::{Blockchain, SledDb};


fn main() {
    tracing_subscriber::fmt().init();

    let path = current_dir().unwrap().join("data");
    let mut bc = Blockchain::new(SledDb::new(path));

    bc.mine_block("Justin -> Bob 2 btc");
    bc.mine_block("Justin -> Bruce 2 btc");

    bc.blocks_info();
}