use crate::error::Error;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

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
    let pubkey =
        Pubkey::from_str(address).map_err(|_| Error::InvalidAddress)?;
    Ok(pubkey)
}
