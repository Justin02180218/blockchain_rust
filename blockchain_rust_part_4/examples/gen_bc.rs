use std::{env::current_dir, sync::Arc};

use blockchain_rust_part_4::{Blockchain, SledDb, UTXOSet};


fn main() {
    tracing_subscriber::fmt().init();

    let genesis_addr = "Justin";

    let path = current_dir().unwrap().join("data");
    let storage = Arc::new(SledDb::new(path));

    let bc = Blockchain::new(storage.clone(), genesis_addr);
    let utxos = UTXOSet::new(storage);
    utxos.reindex(&bc).unwrap();

    bc.blocks_info();
}