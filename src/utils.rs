use crate::error::Error;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::EncodableKey};
use std::{path::Path, str::FromStr};

#[derive(Debug, Clone)]
pub enum Chains {
    DevNet,
    Testnet,
    Mainnet,
}

impl TryFrom<&str> for Chains {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "dev" => Ok(Chains::DevNet),
            "testnet" => Ok(Chains::Testnet),
            "mainnet" => Ok(Chains::Mainnet),
            _ => Err(Error::InvalidNetwork),
        }
    }
}

pub fn convert_address_to_pubkey(address: &str) -> Result<Pubkey, Error> {
    let pubkey = Pubkey::from_str(address).map_err(|_| Error::InvalidAddress)?;
    Ok(pubkey)
}

pub fn open_keypair_file(path: &Path) -> Result<Keypair, Error> {
    let keypair = Keypair::read_from_file(path)
        .map_err(|_| Error::IO(std::io::ErrorKind::NotFound.into()))?;

    Ok(keypair)
}
