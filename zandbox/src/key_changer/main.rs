//!
//! The zkSync account public key changer.
//!

use colored::Colorize;

static PRIVATE_KEY: &str = "7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110";

const NETWORK: zksync::Network = zksync::Network::Localhost;

///
/// The utility entry point.
///
#[actix_rt::main]
async fn main() {
    let provider = zksync::Provider::new(NETWORK);
    let private_key: zksync_types::H256 = PRIVATE_KEY
        .parse()
        .expect(zinc_const::panic::DATA_CONVERSION);
    let address = zksync_types::tx::PackedEthSignature::address_from_private_key(&private_key)
        .expect(zinc_const::panic::DATA_CONVERSION);
    let wallet_credentials = zksync::WalletCredentials::from_eth_signer(
        address,
        zksync_eth_signer::PrivateKeySigner::new(private_key),
        NETWORK,
    )
    .await
    .expect("Wallet credentials");
    let wallet = zksync::Wallet::new(provider, wallet_credentials)
        .await
        .expect("Wallet initialization");

    let tx_info = wallet
        .start_change_pubkey()
        .fee(1_000_000_000_000_000_000_u64)
        .fee_token("ETH")
        .expect("Fee token resolving")
        .send()
        .await
        .expect("Transaction sending")
        .wait_for_commit()
        .await
        .expect("Transaction waiting");
    if !tx_info.success.unwrap_or_default() {
        panic!(tx_info
            .fail_reason
            .unwrap_or_else(|| "Unknown error".to_owned()));
    }

    println!("{}", "OK".bright_green());
}
