//!
//! The contract resource POST method `initialize` module.
//!

use std::time::Duration;

use actix_web::http::StatusCode;
use actix_web::web;

use zksync::provider::Provider;

use crate::database::error::Error as DatabaseError;
use crate::database::model;
use crate::error::Error;
use crate::response::Response;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Get the contract from the in-memory locked contracts list.
/// 2. Make the initial deposit to the newly created contract.
/// 3. Send the change-pubkey transaction for the contract.
/// 4. Set the received contract account ID.
/// 5. Write the contract and its storage to the persistent database.
///
pub async fn handle(
    app_data: crate::WebData,
    query: web::Query<zinc_types::InitializeRequestQuery>,
    body: web::Json<zinc_types::InitializeRequestBody>,
) -> crate::Result<zinc_types::InitializeResponseBody, Error> {
    let query = query.into_inner();
    let body = body.into_inner();
    let log_id = serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION);

    let postgresql = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql
        .clone();
    let network = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .network;

    log::info!("[{}] Unlocking sequence started", log_id);

    let mut contract = app_data
        .write()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .locked_contracts
        .remove(&query.address)
        .ok_or_else(|| {
            Error::ContractNotFound(
                serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION),
            )
        })?;

    if let zksync_types::ZkSyncTx::Transfer(ref transfer) = body.transaction.tx {
        let token = contract
            .wallet
            .tokens
            .resolve(transfer.token.into())
            .ok_or_else(|| Error::TokenNotFound(transfer.token.to_string()))?;

        log::info!(
            "[{}] Sending {} {} from {} to {} with fee {}",
            log_id,
            zksync_utils::format_ether(&transfer.amount),
            token.symbol,
            serde_json::to_string(&transfer.from).expect(zinc_const::panic::DATA_CONVERSION),
            serde_json::to_string(&transfer.to).expect(zinc_const::panic::DATA_CONVERSION),
            zksync_utils::format_ether(&transfer.fee),
        );
    }

    let fee_token_id = match body.transaction.tx {
        zksync_types::ZkSyncTx::Transfer(ref transfer) => transfer.token,
        _ => panic!(zinc_const::panic::VALUE_ALWAYS_EXISTS),
    };

    let tx_info = contract
        .wallet
        .provider
        .send_tx(
            body.transaction.tx,
            body.transaction
                .ethereum_signature
                .map(|signature| signature.signature),
        )
        .await
        .map(|tx_hash| {
            let mut handle = zksync::operations::SyncTransactionHandle::new(
                tx_hash,
                contract.wallet.provider.clone(),
            )
            .commit_timeout(Duration::from_secs(10));
            handle
                .polling_interval(Duration::from_millis(200))
                .expect(zinc_const::panic::DATA_CONVERSION);
            handle
        })?
        .wait_for_commit()
        .await?;
    if !tx_info.success.unwrap_or_default() {
        return Err(Error::TransferFailure(
            tx_info
                .fail_reason
                .unwrap_or_else(|| "Unknown error".to_owned()),
        ));
    }

    let account_id = zksync::utils::wait_for_account_id(&mut contract.wallet, 10_000)
        .await
        .ok_or(Error::AccountIdNotFound)?;

    log::info!("[{}] Sending the change-pubkey transaction", log_id);
    let mut change_pubkey = contract.wallet.start_change_pubkey();
    if let zksync::Network::Rinkeby = network {
        change_pubkey = change_pubkey.fee(0u64);
    } else {
        change_pubkey =
            change_pubkey.fee(zinc_types::num_compat_backward(contract.change_pubkey_fee));
    }
    let mut handle = change_pubkey
        .fee_token(fee_token_id)?
        .send()
        .await?
        .commit_timeout(Duration::from_secs(10));
    handle
        .polling_interval(Duration::from_millis(200))
        .expect(zinc_const::panic::DATA_CONVERSION);
    let tx_info = handle.wait_for_commit().await?;
    if !tx_info.success.unwrap_or_default() {
        return Err(Error::ChangePubkey(
            tx_info
                .fail_reason
                .unwrap_or_else(|| "Unknown error".to_owned()),
        ));
    }

    {
        let mut transaction = postgresql.new_transaction().await?;

        match postgresql
            .select_project(
                model::project::select_one::Input::new(
                    contract.name.clone(),
                    contract.version.clone(),
                ),
                Some(&mut transaction),
            )
            .await
        {
            Ok(output) => {
                if output.project
                    != serde_json::to_value(&contract.project)
                        .expect(zinc_const::panic::DATA_CONVERSION)
                {
                    return Err(Error::ContractSourceCodeMismatch);
                }
            }
            Err(DatabaseError::NotFound { .. }) => {
                postgresql
                    .insert_project(
                        model::project::insert_one::Input::new(
                            contract.name.clone(),
                            contract.version.clone(),
                            semver::Version::parse(env!("CARGO_PKG_VERSION"))
                                .expect(zinc_const::panic::DATA_CONVERSION),
                            contract.project,
                            contract.bytecode,
                            contract.verifying_key,
                        ),
                        Some(&mut transaction),
                    )
                    .await?;
            }
            Err(error) => return Err(error.into()),
        };

        postgresql
            .insert_contract(
                model::contract::insert_one::Input::new(
                    account_id,
                    contract.name,
                    contract.version,
                    contract.instance,
                    contract.eth_address,
                    contract.eth_private_key,
                ),
                Some(&mut transaction),
            )
            .await?;

        postgresql
            .insert_fields(
                contract.storage.into_database_insert(account_id),
                Some(&mut transaction),
            )
            .await?;

        transaction.commit().await?;
    }

    let response = zinc_types::InitializeResponseBody::new(account_id);

    log::info!("[{}] Unlocking sequence finished", log_id);
    Ok(Response::new_with_data(StatusCode::OK, response))
}
