//!
//! The contract resource POST method `call` module.
//!

use std::time::Duration;

use actix_web::http::StatusCode;
use actix_web::web;
use num_old::BigUint;
use num_old::Zero;

use zksync::operations::SyncTransactionHandle;
use zksync_types::tx::ZkSyncTx;

use zinc_vm::Bn256;
use zinc_vm::ContractInput;
use zinc_zksync::Transaction;

use crate::contract::Contract;
use crate::error::Error;
use crate::response::Response;
use crate::storage::Storage;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Get the contract and its data from the database.
/// 2. Extract the called method from its metadata and check if it is mutable.
/// 3. Parse the method input arguments.
/// 4. Run the method on the Zinc VM.
/// 5. Create a transactions array from the client and contract transfers.
/// 6. Send the transactions to zkSync and store its handles.
/// 7. Wait for all transactions to be committed.
/// 8. Update the contract storage state in the database.
/// 9. Send the contract method execution result back to the client.
///
pub async fn handle(
    app_data: crate::WebData,
    query: web::Query<zinc_zksync::CallRequestQuery>,
    body: web::Json<zinc_zksync::CallRequestBody>,
) -> crate::Result<serde_json::Value, Error> {
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

    log::info!("[{}] Calling method `{}`", log_id, query.method);

    let contract = Contract::new(network, postgresql.clone(), query.address).await?;

    let method = match contract.build.methods.get(query.method.as_str()).cloned() {
        Some(method) => method,
        None => return Err(Error::MethodNotFound(query.method)),
    };
    if !method.is_mutable {
        return Err(Error::MethodIsImmutable(query.method));
    }

    let arguments = zinc_build::Value::try_from_typed_json(body.arguments, method.input)
        .map_err(Error::InvalidInput)?;

    let method = query.method;
    let contract_build = contract.build;
    let contract_storage = contract.storage;
    let transaction = (&body.transaction).try_to_msg(&contract.wallet)?;
    let vm_time = std::time::Instant::now();
    let output = async_std::task::spawn_blocking(move || {
        zinc_vm::ContractFacade::new(contract_build).run::<Bn256>(ContractInput::new(
            arguments,
            contract_storage.into_build(),
            method,
            transaction,
        ))
    })
    .await
    .map_err(Error::VirtualMachine)?;
    log::info!(
        "[{}] VM executed in {} ms",
        log_id,
        vm_time.elapsed().as_millis()
    );

    let storage = Storage::from_build(output.storage).into_database_update(contract.account_id);

    let mut transactions = Vec::with_capacity(1 + output.transfers.len());
    let client_eth_signature = body.transaction.ethereum_signature.signature.clone();
    if let ZkSyncTx::Transfer(ref transfer) = body.transaction.tx {
        let token = contract
            .wallet
            .tokens
            .resolve(transfer.token.into())
            .ok_or_else(|| Error::TokenNotFound(transfer.token.to_string()))?;

        log::info!(
            "[{}] Sending {} {} from {} to {} with total batch fee {} {}",
            log_id,
            zksync_utils::format_units(&transfer.amount, token.decimals),
            token.symbol,
            serde_json::to_string(&transfer.from).expect(zinc_const::panic::DATA_CONVERSION),
            serde_json::to_string(&transfer.to).expect(zinc_const::panic::DATA_CONVERSION),
            zksync_utils::format_units(&transfer.fee, token.decimals),
            token.symbol,
        );
    }
    transactions.push(body.transaction);
    let mut nonce = contract
        .wallet
        .provider
        .account_info(query.address)
        .await?
        .committed
        .nonce;
    for transfer in output.transfers.into_iter() {
        let recipient = transfer.recipient.into();
        let token = contract
            .wallet
            .tokens
            .resolve(
                zinc_zksync::eth_address_from_vec(transfer.token_address.to_bytes_be().to_vec())
                    .into(),
            )
            .ok_or_else(|| {
                Error::TokenNotFound(
                    transfer
                        .token_address
                        .to_str_radix(zinc_const::base::HEXADECIMAL),
                )
            })?;
        let amount = zksync::utils::closest_packable_token_amount(
            &zinc_zksync::num_compat_backward(transfer.amount),
        );
        let fee = BigUint::zero();

        log::info!(
            "[{}] Sending {} {} from {} to {}",
            log_id,
            zksync_utils::format_units(&amount, token.decimals),
            token.symbol,
            serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION),
            serde_json::to_string(&recipient).expect(zinc_const::panic::DATA_CONVERSION),
        );

        let (transfer, signature) = contract
            .wallet
            .signer
            .sign_transfer(token, amount, fee, recipient, nonce)
            .await?;
        transactions.push(Transaction::new(
            ZkSyncTx::Transfer(Box::new(transfer)),
            signature.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
        ));

        nonce += 1;
    }

    let provider = contract.wallet.provider;
    let handles: Vec<SyncTransactionHandle> = provider
        .send_txs_batch(
            transactions
                .into_iter()
                .map(|transaction| {
                    (
                        transaction.tx,
                        Some(transaction.ethereum_signature.signature),
                    )
                })
                .collect(),
            Some(client_eth_signature),
        )
        .await?
        .into_iter()
        .map(|tx_hash| {
            let mut handle = SyncTransactionHandle::new(tx_hash, provider.clone())
                .commit_timeout(Duration::from_secs(10));
            handle
                .polling_interval(Duration::from_millis(200))
                .expect("Validated inside the method");
            handle
        })
        .collect();

    if let Some(handle) = handles.last() {
        let tx_info = handle.wait_for_commit().await?;
        if !tx_info.success.unwrap_or_default() {
            return Err(Error::TransferFailure(
                tx_info
                    .fail_reason
                    .unwrap_or_else(|| "Unknown error".to_owned()),
            ));
        }
    }

    postgresql.update_fields(storage, None).await?;

    let response = serde_json::json!({
        "output": output.result.into_json(),
    });

    log::info!("[{}] Call finished", log_id);
    Ok(Response::new_with_data(StatusCode::OK, response))
}
