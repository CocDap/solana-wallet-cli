use std::path::Path;

use clap::{Parser, Subcommand};

pub mod error;
pub mod utils;
use error::{Error, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, native_token::lamports_to_sol, signature::Keypair, signer::{EncodableKey, Signer}, system_transaction};

use utils::{convert_address_to_pubkey, open_keypair_file, Chains};


#[derive(Parser, Debug)]
#[command(author ="CocDap", version ="0.0.1", about = "Simple local solana wallet")]
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
            let file_path = match path {
                Some(p) => p,
                None => "keypair.json".to_string()

            };
            let wallet = Keypair::new();
            let pubkey = wallet.pubkey();
            // convert to bs58 address
            let address = bs58::encode(pubkey).into_string();


            println!("Your address is :{}", address);
            // write encode keypair
            let _ = wallet
                .write_to_file(file_path)
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
        Commands::Transfer { to, amount, path } => {
            // load keypair 
            let p = Path::new(path.as_str());
            let keypair = open_keypair_file(p)?;
            let to_pubkey = convert_address_to_pubkey(to.as_str())?;
            let latest_block_hash = client.get_latest_blockhash().unwrap();

            // Perform transaction transfer
            let tx = system_transaction::transfer(&keypair, &to_pubkey, amount, latest_block_hash);
            let signature = client.send_and_confirm_transaction(&tx).map_err(error::Error::TransactionError)?;
            println!("Signature:{signature}");
        },
    }

    Ok(())
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Generate local keypair")]
    Generate {
        #[clap(long, help = "Path to save keypair.json")]
        path: Option<String>,
    },
    #[clap(about = "Get free SOL")]
    Faucet {
        #[clap(long)]
        address: String,
        #[clap(long)]
        amount: u64,
    },
    #[clap(about = "Transfer some SOLs from sender(keypair) to receiver")]
    Transfer {
        #[clap(long)]
        to: String,
        #[clap(long)]
        amount: u64,
        #[clap(long)]
        path: String
    },
    #[clap(about = "Check SOL balance")]
    Balance {
        #[clap(long)]
        address: String,
    },
}
