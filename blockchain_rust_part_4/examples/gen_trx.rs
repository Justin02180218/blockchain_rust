use std::{env::current_dir, sync::Arc};

use blockchain_rust_part_4::{Blockchain, SledDb, UTXOSet, Transaction};


fn main() {
    tracing_subscriber::fmt().init();

    let justin_addr = "Justin";
    let bob_addr = "Bob";
    // let bruce_addr = "Bruce";

    let path = current_dir().unwrap().join("data");
    let storage = Arc::new(SledDb::new(path));

    let mut bc = Blockchain::new(storage.clone(), justin_addr);
    let utxos = UTXOSet::new(storage);

    let tx_1 = Transaction::new_utxo(justin_addr, bob_addr, 4, &utxos);
    // let tx_2 = Transaction::new_utxo(justin_addr, bruce_addr, 2, &utxos);

    let txs = vec![tx_1];

    bc.mine_block(&txs);
    utxos.reindex(&bc).unwrap();

    bc.blocks_info();
}