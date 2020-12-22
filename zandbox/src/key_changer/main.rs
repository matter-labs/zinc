//!
//! The zkSync account public key changer.
//!

pub(crate) mod arguments;

use colored::Colorize;

use self::arguments::Arguments;

const NETWORK: zksync::Network = zksync::Network::Localhost;

///
/// The utility entry point.
///
#[actix_rt::main]
async fn main() {
    let args = Arguments::new();

    let provider = zksync::RpcProvider::new(NETWORK);
    let private_key: zksync_types::H256 = args.private_key["0x".len()..]
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

    println!("     {} {}", "Changed".bright_green(), args.private_key);
}
