
use solana_client::client_error::ClientError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Not supported network")]
    InvalidNetwork,
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Convert Error")]
    InvalidAddress,
    #[error("Error Requesting Airdrop {0}")]
    RequestAirDrop(String),
    #[error("Transaction Failed")]
    TransactionError(ClientError)



}

impl From<ClientError> for Error {
    fn from(e: ClientError) -> Self {
        Self::TransactionError(e)
    }
}