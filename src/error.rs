

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Not supported network")]
    InvalidNetwork,
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Convert Error")]
    ConversionPubkeyError,
    #[error("Error Requesting Airdrop {0}")]
    RequestAirDrop(String)


}