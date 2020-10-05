//!
//! The zkSync account public key changer.
//!

use colored::Colorize;

static TOKEN_SYMBOL: &str = "ETH";
static ETH_ADDRESS: &str = "7645e91c4Fa80b42eE532Bc53b5370F860e8daC9";
static ETH_PRIVATE_KEY: &str = "d4f64abbedb7da5938abf9ef9b3ae6446bf960680c150547d469618ca5f23e3b";

///
/// The utility entry point.
///
#[actix_rt::main]
async fn main() {
    let provider = zksync::Provider::new(zksync::Network::Rinkeby);
    let wallet_credentials = zksync::WalletCredentials::from_eth_pk(
        ETH_ADDRESS.parse().expect("ETH address parsing"),
        ETH_PRIVATE_KEY.parse().expect("ETH private key parsing"),
        zksync::Network::Rinkeby,
    )
    .expect("Wallet credentials");
    let wallet = zksync::Wallet::new(provider, wallet_credentials)
        .await
        .expect("Wallet initialization");

    let tx_info = wallet
        .start_change_pubkey()
        .fee_token(TOKEN_SYMBOL)
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
            .unwrap_or_else(|| "Unknown error".to_owned()),);
    }

    println!("{}", "OK".bright_green());
}
