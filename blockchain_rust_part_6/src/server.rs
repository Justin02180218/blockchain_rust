use std::{env::{current_dir, self}, sync::Arc};

use anyhow::Result;
use blockchain_rust_part_6::{Node, SledDb};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut path = String::from("data");
    if let Some(args) = env::args().nth(2) {
        path = args;
    }

    let path = current_dir().unwrap().join(path);
    let db = Arc::new(SledDb::new(path));
    let mut node = Node::new(db).await?;
    node.start().await?;
    Ok(())
}