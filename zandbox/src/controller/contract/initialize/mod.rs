//!
//! The contract resource POST method `initialize` module.
//!

pub mod error;
pub mod request;
pub mod response;

use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use actix_web::http::StatusCode;
use actix_web::web;

use zksync::operations::SyncTransactionHandle;
use zksync_types::tx::ZkSyncTx;

use crate::database::model::contract::insert_new::Input as ContractInsertNewInput;
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
/// 3. Send the change-pubkey transaction for the contract.
/// 4. Set the received contract account ID.
/// 5. Write the contract and its storage to the persistent database.
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
        .postgresql
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
    let wallet_credentials = zksync::WalletCredentials::from_eth_signer(
        query.address,
        zksync_eth_signer::EthereumSigner::from_key(contract.eth_private_key),
        query.network,
    )
    .await?;
    let mut wallet = zksync::Wallet::new(provider, wallet_credentials).await?;

    if let ZkSyncTx::Transfer(ref transfer) = body.transaction.tx {
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

    let fee_token_id = match body.transaction.tx {
        ZkSyncTx::Transfer(ref transfer) => transfer.token,
        _ => panic!(zinc_const::panic::VALUE_ALWAYS_EXISTS),
    };

    let tx_info = wallet
        .provider
        .send_tx(
            body.transaction.tx,
            Some(body.transaction.ethereum_signature.signature),
        )
        .await
        .map(|tx_hash| {
            let mut handle = SyncTransactionHandle::new(tx_hash, wallet.provider.clone())
                .commit_timeout(Duration::from_secs(10));
            handle
                .polling_interval(Duration::from_millis(200))
                .expect("Validated inside the method");
            handle
        })?
        .wait_for_commit()
        .await?;
    if !tx_info.success.unwrap_or_default() {
        return Err(Error::InitialTransfer(
            tx_info
                .fail_reason
                .unwrap_or_else(|| "Unknown error".to_owned()),
        ));
    }

    log::debug!("Waiting for the account ID");
    let account_id = zksync::utils::wait_for_account_id(&mut wallet, 10_000)
        .await
        .ok_or(Error::AccountId)?;

    log::debug!("Sending the change-pubkey transaction");
    let mut handle = wallet
        .start_change_pubkey()
        .fee(0_u64)
        .fee_token(fee_token_id)?
        .send()
        .await?
        .commit_timeout(Duration::from_secs(10));
    handle
        .polling_interval(Duration::from_millis(200))
        .expect("Validated inside the method");
    let tx_info = handle.wait_for_commit().await?;
    if !tx_info.success.unwrap_or_default() {
        return Err(Error::ChangePubkey(
            tx_info
                .fail_reason
                .unwrap_or_else(|| "Unknown error".to_owned()),
        ));
    }

    log::debug!("Setting the contract account ID to {}", account_id);
    app_data
        .write()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .contracts
        .get_mut(&query.address)
        .ok_or_else(|| {
            Error::ContractNotFound(
                serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION),
            )
        })?
        .set_account_id(account_id);

    log::debug!("Writing the contract to the persistent PostgreSQL database");
    postgresql
        .insert_contract(ContractInsertNewInput::new(
            account_id,
            contract.name,
            contract.version,
            contract.instance,
            env!("CARGO_PKG_VERSION").to_owned(),
            contract.source_code,
            contract.bytecode,
            contract.verifying_key,
            contract.eth_address,
            contract.eth_private_key,
        ))
        .await?;

    log::debug!("Writing the contract storage to the persistent PostgreSQL database");
    postgresql
        .insert_fields(contract.storage.into_database_insert(account_id))
        .await?;

    let response = ResponseBody::new(account_id);

    log::debug!("The contract has been unlocked and published");
    Ok(Response::new_with_data(StatusCode::OK, response))
}
