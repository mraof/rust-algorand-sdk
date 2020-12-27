# rust-algorand-sdk

# Documentation

General Algorand documentation is available at https://developer.algorand.org/
This is the current attempt to stay up Rust version of the Algorand SDK

# Quickstart
This quick start guide assumes the user has the Algorand Sandbox 2.0 installed.

```rust
use algosdk::{AlgodClient};

fn main() {
    let algod_address = "http://localhost:4001";
    let algod_token="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let algod_client = AlgodClient::new(algod_address, algod_token);

    println!("Algod versions: {:?}", algod_client.versions().unwrdap().versions);
    println!("Algod status: {:?}", algod_client.status().unwrap());
}
```
