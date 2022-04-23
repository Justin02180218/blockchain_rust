use std::{env::current_dir, sync::Arc};

use blockchain_rust_part_5::{Blockchain, SledDb, UTXOSet, Wallets};


fn main() {
    tracing_subscriber::fmt().init();

    let mut wallets = Wallets::new().unwrap();
    let genesis_addr = wallets.create_wallet();
    println!("==> genesis address: {}", genesis_addr);

    let path = current_dir().unwrap().join("data");
    let storage = Arc::new(SledDb::new(path));

    let bc = Blockchain::new(storage.clone(), &genesis_addr);
    let utxos = UTXOSet::new(storage);
    utxos.reindex(&bc).unwrap();

    bc.blocks_info();
}