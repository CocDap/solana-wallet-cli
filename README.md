
# Solana Fellowship Program 2024 - Module 1 

## Solana Wallet CLI 
Make a simple local solana wallet cli 

## Features
- [x] Generate local keypair
- [x] Faucet SOL 
- [x] Check SOL Balance 
- [x] Transfer transaction 

## How to run

- Build the project
  ```bash
  cargo build --release
  ```
- Run the project
  ```bash
  ./target/release/solana-wallet-cli --help 
  ```

## Usage

```rust
Usage: solana-wallet-cli [OPTIONS] <COMMAND>

Commands:
  generate  
  faucet    
  transfer  
  balance   
  help      Print this message or the help of the given subcommand(s)

Options:
  -e <ENVIRONMENT>      [default: dev]
  -h, --help            Print help
```
