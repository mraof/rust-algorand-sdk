use std::error::Error;

use algosdk::{mnemonic, KmdClient};

fn main() -> Result<(), Box<dyn Error>> {
    let kmd_address = "http://localhost:4001";
    let kmd_token = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    let kmd_client = KmdClient::new(kmd_address, kmd_token);

    let list_response = kmd_client.list_wallets()?;

    let wallet_id = match list_response
        .wallets
        .into_iter()
        .find(|wallet| wallet.name == "testwallet")
    {
        Some(wallet) => wallet.id,
        None => return Err("Wallet not found".into()),
    };

    let init_response = kmd_client.init_wallet_handle(&wallet_id, "testpassword")?;
    let wallet_handle_token = init_response.wallet_handle_token;

    let export_response =
        kmd_client.export_master_derivation_key(&wallet_handle_token, "testpassword")?;
    let mdk = export_response.master_derivation_key;

    // String representation of the mdk, keep in safe place and don't share it
    let string_to_save = mnemonic::from_key(&mdk.0)?;

    println!("Backup phrase: {}", string_to_save);

    Ok(())
}
