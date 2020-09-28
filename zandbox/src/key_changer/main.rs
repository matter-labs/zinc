//!
//! The zkSync account public key changer.
//!

///
/// The utility entry point.
///
#[actix_rt::main]
async fn main() {
    let provider = zksync::Provider::new(zksync::Network::Localhost);
    let wallet_credentials = zksync::WalletCredentials::from_eth_pk(
        "36615Cf349d7F6344891B1e7CA7C72883F5dc049".parse().expect("ETH address parsing"),
        "7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110".parse().expect("ETH private key parsing"),
        zksync::Network::Localhost,
    ).expect("Wallet credentials");
    let wallet = zksync::Wallet::new(provider, wallet_credentials).await.expect("Wallet initialization");

    let tx_info = wallet
        .start_change_pubkey()
        .send()
        .await.expect("Transaction sending")
        .wait_for_commit()
        .await.expect("Transaction waiting");
    if !tx_info.success.unwrap_or_default() {
        panic!(tx_info
                .fail_reason
                .unwrap_or_else(|| "Unknown error".to_owned()),
        );
    }

    println!("OK");
}
