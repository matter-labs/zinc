//!
//! The contract resource POST method `call` module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use num_old::BigUint;
use num_old::Zero;
use serde_json::json;
use serde_json::Value as JsonValue;

use zksync::operations::SyncTransactionHandle;
use zksync::zksync_models::FranklinTx;

use zinc_build::ContractFieldValue as BuildContractFieldValue;
use zinc_build::Value as BuildValue;
use zinc_data::Transaction;
use zinc_data::Transfer;
use zinc_vm::Bn256;

use crate::database::model::field::select::Input as FieldSelectInput;
use crate::database::model::field::select::Output as FieldSelectOutput;
use crate::database::model::field::update::Input as FieldUpdateInput;
use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Body as RequestBody;
use self::request::Query as RequestQuery;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Get the contract from the in-memory cache.
/// 2. Extract the called method from its metadata and check if it is mutable.
/// 3. Check if the transactions in the contract method arguments match the signed ones.
/// 4. Parse the method input arguments.
/// 5. Get the contract storage from the database and convert it to the Zinc VM representation.
/// 6. Run the method on the Zinc VM.
/// 7. Extract the storage with the updated state from the Zinc VM.
/// 8. Create a transactions array from the client and contract transfers.
/// 9. Send the transactions to zkSync and store its handles.
/// 10. Wait for all transactions to be committed.
/// 11. Update the contract storage state in the database.
/// 12. Send the contract method execution result back to the client.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<RequestQuery>,
    body: web::Json<RequestBody>,
) -> crate::Result<JsonValue, Error> {
    let query = query.into_inner();
    let body = body.into_inner();

    let postgresql = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql_client
        .clone();

    log::debug!(
        "Calling method `{}` of contract {}",
        query.method,
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

    let method = match contract.build.methods.get(query.method.as_str()).cloned() {
        Some(method) => method,
        None => return Err(Error::MethodNotFound(query.method)),
    };
    if !method.is_mutable {
        return Err(Error::MethodIsImmutable(query.method));
    }

    log::debug!("Initializing the contract wallet");
    let provider = zksync::Provider::new(query.network);
    let wallet_credentials = zksync::WalletCredentials::from_eth_pk(
        query.address,
        contract.eth_private_key,
        query.network,
    )?;
    let wallet = zksync::Wallet::new(provider, wallet_credentials).await?;

    let argument_transfer = Transfer::try_from_json(&body.arguments)?;
    argument_transfer.validate(&wallet, &body.transaction)?;

    let input_value = BuildValue::try_from_typed_json(body.arguments, method.input)
        .map_err(Error::InvalidInput)?;

    log::debug!("Loading the pre-transaction contract storage");
    let storage_value = postgresql
        .select_fields(FieldSelectInput::new(query.address))
        .await?;
    let storage_fields_count = storage_value.len();
    let mut fields = Vec::with_capacity(storage_value.len());
    for (index, FieldSelectOutput { name, value }) in storage_value.into_iter().enumerate() {
        let r#type = contract.build.storage[index].r#type.clone();
        let value = BuildValue::try_from_typed_json(value, r#type)
            .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);
        fields.push(BuildContractFieldValue::new(
            name,
            value,
            contract.build.storage[index].is_public,
            contract.build.storage[index].is_external,
        ));
    }
    let storage_value = BuildValue::Contract(fields);

    log::debug!("Running the contract method on the virtual machine");
    let method = query.method;
    let contract_build = contract.build;
    let vm_time = std::time::Instant::now();
    let output = async_std::task::spawn_blocking(move || {
        zinc_vm::ContractFacade::new(contract_build).run::<Bn256>(
            input_value,
            storage_value,
            method,
        )
    })
    .await
    .map_err(Error::RuntimeError)?;
    log::debug!("VM executed in {} ms", vm_time.elapsed().as_millis());

    log::debug!("Loading the post-transaction contract storage");
    let mut storage_fields = Vec::with_capacity(storage_fields_count);
    match output.storage {
        BuildValue::Contract(fields) => {
            for (index, field) in fields.into_iter().enumerate() {
                storage_fields.push(FieldUpdateInput::new(
                    query.address,
                    index as i16,
                    field.value.into_json(),
                ));
            }
        }
        _ => panic!(zinc_const::panic::VALIDATED_DURING_RUNTIME_EXECUTION),
    }

    log::debug!("Building the transaction list");
    let mut transactions = Vec::with_capacity(1 + output.transfers.len());
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
    transactions.push(body.transaction);
    let mut nonce = wallet
        .provider
        .account_info(query.address)
        .await?
        .committed
        .nonce;
    for transfer in output.transfers.into_iter() {
        let recipient = transfer.recipient.into();
        let token = wallet
            .tokens
            .resolve(transfer.token_id.into())
            .ok_or(Error::TokenNotFound(transfer.token_id))?;
        let amount =
            zksync::utils::closest_packable_token_amount(&num_old::BigUint::from_bytes_be(
                transfer.amount.to_bytes_be().as_slice(), // TODO: remove when the SDK is updated
            ));
        let fee = BigUint::zero();

        log::debug!(
            "Sending {} {} from {} to {} with fee {}",
            zksync_utils::format_ether(&amount),
            token.symbol,
            serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION),
            serde_json::to_string(&recipient).expect(zinc_const::panic::DATA_CONVERSION),
            zksync_utils::format_ether(&fee),
        );

        let (transfer, signature) = wallet
            .signer
            .sign_transfer(token, amount, fee, recipient, nonce)?;
        transactions.push(Transaction::new(
            FranklinTx::Transfer(Box::new(transfer)),
            signature.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
        ));

        nonce += 1;
    }

    log::debug!(
        "Sending the transactions to zkSync on network `{}`",
        query.network
    );
    let handles = wallet
        .provider
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
        )
        .await?
        .into_iter()
        .map(|tx_hash| SyncTransactionHandle::new(tx_hash, wallet.provider.clone()));

    log::debug!("Waiting for the transfers to be committed");
    let mut reasons = Vec::with_capacity(handles.len());
    let mut are_errors = false;
    for handle in handles.into_iter() {
        let tx_info = handle.wait_for_commit().await?;
        if tx_info.success.unwrap_or_default() {
            reasons.push("OK".to_owned());
        } else {
            reasons.push(
                tx_info
                    .fail_reason
                    .unwrap_or_else(|| "Unknown error".to_owned()),
            );
            are_errors = true;
        }
    }
    if are_errors {
        return Err(Error::TransferFailure { reasons });
    }

    log::debug!("Committing the contract storage state to the database");
    postgresql.update_fields(storage_fields).await?;

    let response = json!({
        "output": output.result.into_json(),
    });

    log::debug!("The call has been successfully executed");
    Ok(Response::new_with_data(StatusCode::OK, response))
}
