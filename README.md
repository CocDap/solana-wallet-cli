
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
Simple local solana wallet

Usage: solana-wallet-cli [OPTIONS] <COMMAND>

Commands:
  generate  Generate local keypair
  faucet    Get free SOL
  transfer  Transfer some SOLs from sender(keypair) to receiver
  balance   Check SOL balance
  help      Print this message or the help of the given subcommand(s)

Options:
  -e <ENVIRONMENT>      [default: dev]
  -h, --help            Print help
  -V, --version         Print version
```

## Step 
### Step 1 - Generate keypair 
Noted: 
+ Default path is current directory 
+ Keypair will be encoded


```bash
./target/release/solana-wallet-cli generate                                       
```
Result:
```
Your address is :8MFjBAgoknegjNTycdgom38m8gow4Pk3KkkVZBxahgya
```

### Step 2 - Faucet 
Faucet generated address above  with 1 SOL = 1,000,000,000 lamports

```bash
./target/release/solana-wallet-cli faucet --address 8MFjBAgoknegjNTycdgom38m8gow4Pk3KkkVZBxahgya --amount 1000000000                                    
```

Result:

```
Transaction:JWH9sHg8ueZTCRemTg8gY3mgQSMxTmiVWWciHgqGo9EG859MRSVMMpas5Rrz9UF8tpwcwU4T41mDR29HpkUPiVf Status: true
```

### Step 3 -  Check balance  

```bash
./target/debug/solana-wallet-cli balance --address 8MFjBAgoknegjNTycdgom38m8gow4Pk3KkkVZBxahgya                            
```

Result:

```
Balance of 8MFjBAgoknegjNTycdgom38m8gow4Pk3KkkVZBxahgya is 1 SOL
```

### Step 4 -  Transfer

Make sure:
+ Sender balance is non-zero ( Faucet above)
+ Keypair Path is correct 

```bash
./target/release/solana-wallet-cli transfer --to Bw9qUWRn6BeP3APtoyxtZBwnDTWwhByLzTy2BA9eEsiz --amount 10000000 --path keypair.json
```

Result:
```
Signature:MizFcxg74fx3B4JBB18cvQXqz2FCM5ZAsXkW6QsBugeNBM5DRBGaPYu6h1y2Ns2W5needRgxTCo4xuNgM1GyMgy
```


