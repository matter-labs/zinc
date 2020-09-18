//!
//! The Zandbox server daemon wait functions.
//!

use std::time::Duration;
use std::time::Instant;

use futures::compat::Future01CompatExt;
use futures::StreamExt;

use zksync::error::ClientError;
use zksync::zksync_models::node::AccountId;
use zksync::Wallet;
use zksync::web3::types::H256;

pub(crate) async fn eth_tx(ethereum: &zksync::EthereumProvider, hash: H256) {
    let timeout = Duration::from_secs(60);
    let mut poller = async_std::stream::interval(Duration::from_millis(100));
    let start = Instant::now();

    while ethereum.web3()
        .eth()
        .transaction_receipt(hash)
        .compat()
        .await
        .unwrap()
        .is_none()
    {
        if start.elapsed() > timeout {
            panic!("ETH transaction timeout");
        }

        poller.next().await;
    }
}

pub(crate) async fn account_id(wallet: &mut Wallet) -> Result<AccountId, ClientError> {
    let timeout = Duration::from_secs(60);
    let mut poller = async_std::stream::interval(std::time::Duration::from_millis(100));
    let start = Instant::now();

    while wallet
        .provider
        .account_info(wallet.address())
        .await?
        .id
        .is_none()
    {
        if start.elapsed() > timeout {
            panic!("ZkSync account ID timeout");
        }

        poller.next().await;
    }

    wallet.update_account_id().await?;

    Ok(wallet.account_id().expect("ZkSync account ID was not set"))
}
