//!
//! The contract resource POST method `initialize` module.
//!

pub mod error;
pub mod request;
pub mod response;

use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use std::time::Instant;

use actix_web::http::StatusCode;
use actix_web::web;
use futures::StreamExt;

use zksync::operations::SyncTransactionHandle;
use zksync::zksync_models::FranklinTx;
use zksync::zksync_models::AccountId;

use crate::database::model::contract::update::account_id::input::Input as ContractUpdateAccountIdInput;
use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Body as RequestBody;
use self::request::Query as RequestQuery;
use self::response::Body as ResponseBody;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Get the contract from the in-memory cache.
/// 2. Make the initial zero deposit to the newly created contract.
/// 2. Send the change-pubkey transaction for the contract.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<RequestQuery>,
    body: web::Json<RequestBody>,
) -> crate::Result<ResponseBody, Error> {
    let query = query.into_inner();
    let body = body.into_inner();

    let postgresql = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql_client
        .clone();

    log::debug!(
        "Initializing contract {}",
        serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION),
    );

    let contract = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .contracts
        .get(&query.address)
        .cloned()
        .ok_or_else(|| {
            Error::ContractNotFound(
                serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION),
            )
        })?;

    log::debug!("Initializing the contract wallet");
    let provider = zksync::Provider::new(query.network);
    let wallet_credentials = zksync::WalletCredentials::from_eth_pk(
        query.address,
        contract.eth_private_key,
        query.network,
    )?;
    let mut wallet = zksync::Wallet::new(provider, wallet_credentials).await?;

    if let FranklinTx::Transfer(ref transfer) = body.transaction.tx {
        let token = wallet
            .tokens
            .resolve(transfer.token.into())
            .ok_or(Error::TokenNotFound(transfer.token))?;

        log::debug!(
            "Sending {} {} from {} to {} with fee {}",
            zksync_utils::format_ether(&transfer.amount),
            token.symbol,
            serde_json::to_string(&transfer.from).expect(zinc_const::panic::DATA_CONVERSION),
            serde_json::to_string(&transfer.to).expect(zinc_const::panic::DATA_CONVERSION),
            zksync_utils::format_ether(&transfer.fee),
        );
    }

    let tx_info = wallet
        .provider
        .send_tx(
            body.transaction.tx,
            Some(body.transaction.ethereum_signature.signature),
        )
        .await
        .map(|tx_hash| SyncTransactionHandle::new(tx_hash, wallet.provider.clone()))?.wait_for_commit().await?;
    if !tx_info.success.unwrap_or_default() {
        return Err(Error::InitialTransfer(
            tx_info
                .fail_reason
                .unwrap_or_else(|| "Unknown error".to_owned()),
        ));
    }

    log::debug!("Waiting for the account ID");
    let account_id = wait_for_account_id(&mut wallet, 10_000).await?;

    log::debug!("Sending the change-pubkey transaction");
    let tx_info = wallet
        .start_change_pubkey()
        .send()
        .await?
        .wait_for_commit()
        .await?;
    if !tx_info.success.unwrap_or_default() {
        return Err(Error::ChangePubkey(
            tx_info
                .fail_reason
                .unwrap_or_else(|| "Unknown error".to_owned()),
        ));
    }

    log::debug!("Writing account ID to the persistent PostgreSQL database");
    postgresql
        .update_contract_account_id(ContractUpdateAccountIdInput::new(
            query.address,
            account_id,
        ))
        .await?;

    log::debug!("Writing account ID to the temporary server cache");
    if let Some(contract) = app_data
        .write()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .contracts
        .get_mut(&query.address)
    {
        contract.set_account_id(account_id);
    }

    let response = ResponseBody::new(account_id);

    log::debug!("The contract has been initialized");
    Ok(Response::new_with_data(StatusCode::OK, response))
}

///
/// Waits until there is a zkSync account ID associated with the `wallet` contract.
///
async fn wait_for_account_id(
    wallet: &mut zksync::Wallet,
    timeout_ms: u64,
) -> Result<AccountId, Error> {
    let timeout = Duration::from_millis(timeout_ms);
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
            return Err(Error::AccountId);
        }

        poller.next().await;
    }

    wallet.update_account_id().await?;

    wallet.account_id().ok_or(Error::AccountId)
}
