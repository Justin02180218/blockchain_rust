use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error("Serialize or Deserialize error")]
    SerializeError(#[from] Box<bincode::ErrorKind>),

    #[error("Failed to access sled db")]
    SledError(#[from] sled::Error),
}