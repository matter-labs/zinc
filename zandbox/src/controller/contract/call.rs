//!
//! The contract resource POST method `call` module.
//!

use std::collections::HashMap;
use std::time::Duration;

use actix_web::http::StatusCode;
use actix_web::web;
use num::BigInt;
use num_old::BigUint;
use num_old::Zero;

use zksync::operations::SyncTransactionHandle;
use zksync_types::tx::ZkSyncTx;

use zinc_vm::Bn256;
use zinc_vm::ContractInput;

use crate::contract::Contract;
use crate::database::model;
use crate::error::Error;
use crate::response::Response;
use crate::storage::keeper::Keeper as StorageKeeper;
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

    let eth_address_bigint =
        BigInt::from_bytes_be(num::bigint::Sign::Plus, contract.eth_address.as_bytes());
    let mut arguments = zinc_build::Value::try_from_typed_json(body.arguments, method.input)
        .map_err(Error::InvalidInput)?;
    arguments.insert_contract_instance(eth_address_bigint.clone());

    let method = query.method;
    let contract_build = contract.build;
    let contract_storage = contract.storage;
    let contract_storage_keeper = StorageKeeper::new(postgresql.clone(), network);
    let transaction = (&body.transaction).try_to_msg(&contract.wallet)?;
    let vm_time = std::time::Instant::now();
    let output = async_std::task::spawn_blocking(move || {
        zinc_vm::ContractFacade::new_with_keeper(contract_build, Box::new(contract_storage_keeper))
            .run::<Bn256>(ContractInput::new(
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

    let mut transactions = Vec::with_capacity(1 + output.transfers.len());
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

    let mut mutated_instances = HashMap::with_capacity(output.storages.len());

    let mut nonces = HashMap::with_capacity(output.storages.len());
    for transfer in output.transfers.into_iter() {
        let contract = mutated_instances.entry(transfer.sender).or_insert(
            postgresql
                .select_contract(
                    model::contract::select_one::Input::new(transfer.sender),
                    None,
                )
                .await?,
        );
        let wallet_credentials = zksync::WalletCredentials::from_eth_signer(
            transfer.sender,
            zksync_eth_signer::PrivateKeySigner::new(zinc_zksync::private_key_from_slice(
                contract.eth_private_key.as_slice(),
            )),
            network,
        )
        .await?;
        let wallet =
            zksync::Wallet::new(zksync::Provider::new(network), wallet_credentials).await?;

        let nonce = nonces.entry(transfer.sender).or_insert(
            wallet
                .provider
                .account_info(transfer.sender)
                .await?
                .committed
                .nonce,
        );
        let token = wallet
            .tokens
            .resolve(transfer.token_address.into())
            .ok_or_else(|| {
                Error::TokenNotFound(
                    serde_json::to_string(&transfer.token_address)
                        .expect(zinc_const::panic::DATA_CONVERSION),
                )
            })?;
        let amount = zksync::utils::closest_packable_token_amount(&transfer.amount);
        let fee = BigUint::zero();

        log::info!(
            "[{}] Sending {} {} from {} to {}",
            log_id,
            zksync_utils::format_units(&amount, token.decimals),
            token.symbol,
            serde_json::to_string(&transfer.sender).expect(zinc_const::panic::DATA_CONVERSION),
            serde_json::to_string(&transfer.recipient).expect(zinc_const::panic::DATA_CONVERSION),
        );

        let (transfer, signature) = wallet
            .signer
            .sign_transfer(token, amount, fee, transfer.recipient, *nonce)
            .await?;
        transactions.push(zinc_zksync::Transaction::new(
            ZkSyncTx::Transfer(Box::new(transfer)),
            signature.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
        ));

        *nonce += 1;
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
            None,
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

    let mut transaction = postgresql.new_transaction().await?;
    for (address, storage) in output.storages.into_iter() {
        let address = zinc_zksync::address_from_slice(address.to_bytes_be().1.as_slice());
        let contract = mutated_instances.entry(address).or_insert(
            postgresql
                .select_contract(
                    model::contract::select_one::Input::new(address),
                    Some(&mut transaction),
                )
                .await?,
        );
        let storage = Storage::from_build(storage)
            .into_database_update(contract.account_id as zksync_types::AccountId);
        postgresql
            .update_fields(storage, Some(&mut transaction))
            .await?;
    }
    transaction.commit().await?;

    let response = serde_json::json!({
        "output": output.result.into_json(),
    });

    log::info!("[{}] Call finished", log_id);
    Ok(Response::new_with_data(StatusCode::OK, response))
}
