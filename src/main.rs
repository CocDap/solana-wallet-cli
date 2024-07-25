use clap::{Parser, Subcommand};

pub mod error;
pub mod utils;
use error::{Error, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, native_token::lamports_to_sol, signature::Keypair, signer::EncodableKey};

use utils::{convert_address_to_pubkey, Chains};


#[derive(Parser, Debug)]
struct Args {
    #[clap(short, default_value = "dev")]
    environment: Option<String>,
    #[command(subcommand)]
    cmd: Commands,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let mut rpc_endpoint = "https://api.devnet.solana.com";

    if let Some(e) = args.environment {
        let chain = Chains::try_from(e.as_str())?;
        match chain {
            Chains::Testnet => rpc_endpoint = "https://api.testnet.solana.com",
            Chains::Mainnet => rpc_endpoint = "https://api.mainnet-beta.solana.com",
            Chains::DevNet => (),
        }
    }

    // Connect RPC
    let client = RpcClient::new_with_commitment(rpc_endpoint, CommitmentConfig::confirmed());

    // Generate key pair
    // Store local folder

    match args.cmd {
        Commands::Generate { path } => {
            let wallet = Keypair::new();
            let _ = wallet
                .write_to_file("keypair.json")
                .map_err(|_| error::Error::IO(std::io::ErrorKind::NotFound.into()))?;
        }
        Commands::Faucet { address, amount } => {
            let pubkey = convert_address_to_pubkey(address.as_str())?;
            match client.request_airdrop(&pubkey, amount) {
                Ok(sig) => loop {
                    if let Ok(confirmed) = client.confirm_transaction(&sig) {
                        if confirmed {
                            println!("Transaction:{} Status: {}", sig, confirmed);
                            return Ok(());
                        }
                    }
                },
                Err(e) => Error::RequestAirDrop(e.to_string()),
            };

        }
        Commands::Balance { address } => {
            let pubkey = convert_address_to_pubkey(address.as_str())?;
            let balance = client.get_account(&pubkey).unwrap().lamports;
            println!("Balance of {address} is {} SOL", lamports_to_sol(balance));


        },
        Commands::Transfer { from, to, amount } => todo!(),
    }

    Ok(())
}

#[derive(Subcommand, Debug)]
enum Commands {
    Generate {
        #[clap(long)]
        path: Option<String>,
    },
    Faucet {
        #[clap(long)]
        address: String,
        #[clap(long)]
        amount: u64,
    },
    Transfer {
        #[clap(long)]
        from: String,
        #[clap(long)]
        to: String,
        #[clap(long)]
        amount: u64,
    },
    Balance {
        #[clap(long)]
        address: String,
    },
}
