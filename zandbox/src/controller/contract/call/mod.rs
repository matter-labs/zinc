//!
//! The contract resource POST method `call` module.
//!

pub mod error;
pub mod request;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use serde_json::Value as JsonValue;

use zksync::operations::SyncTransactionHandle;
use zksync::web3::types::H160;
use zksync::zksync_models::node::FranklinTx;

use zinc_build::ContractFieldValue as BuildContractFieldValue;
use zinc_build::Value as BuildValue;
use zinc_vm::Bn256;

use crate::database::model::field::select::input::Input as FieldSelectInput;
use crate::database::model::field::select::output::Output as FieldSelectOutput;
use crate::database::model::field::update::input::Input as FieldUpdateInput;
use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Body as RequestBody;
use self::request::Query as RequestQuery;

///
/// The HTTP request handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<RequestQuery>,
    body: web::Json<RequestBody>,
) -> crate::Result<JsonValue, Error> {
    let query = query.into_inner();
    let body = body.into_inner();

    log::debug!(
        "Calling method `{}` of contract #{}",
        query.method,
        query.account_id
    );

    let contract = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .contracts
        .get(&query.account_id)
        .cloned()
        .ok_or(Error::ContractNotFound)?;

    let method = contract
        .build
        .methods
        .get(query.method.as_str())
        .cloned()
        .ok_or(Error::MethodNotFound)?;
    if !method.is_mutable {
        return Err(Error::MethodIsImmutable);
    }

    let input_value = BuildValue::try_from_typed_json(body.arguments, method.input)
        .map_err(Error::InvalidInput)?;

    log::debug!("Loading the pre-transaction contract storage");
    let storage_value = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql_client
        .select_fields(FieldSelectInput::new(query.account_id as i64))
        .await
        .map_err(Error::Database)?;
    let storage_fields_count = storage_value.len();
    assert_eq!(
        storage_fields_count,
        contract.build.storage.len(),
        "The database contract storage is corrupted"
    );
    let mut fields = Vec::with_capacity(storage_value.len());
    for (index, FieldSelectOutput { name, value }) in storage_value.into_iter().enumerate() {
        let r#type = contract.build.storage[index].r#type.clone();
        let value = BuildValue::try_from_typed_json(value, r#type)
            .expect("The database contract storage is corrupted");
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
    let output = async_std::task::spawn_blocking(move || {
        zinc_vm::ContractFacade::new(contract_build).run::<Bn256>(
            input_value,
            storage_value,
            method,
        )
    })
    .await
    .map_err(Error::RuntimeError)?;

    log::debug!("Loading the post-transaction contract storage");
    let mut storage_fields = Vec::with_capacity(storage_fields_count);
    match output.storage {
        BuildValue::Contract(fields) => {
            for (index, field) in fields.into_iter().enumerate() {
                storage_fields.push(FieldUpdateInput::new(
                    index as i16,
                    query.account_id as i64,
                    field.value.into_json(),
                ));
            }
        }
        _ => panic!(zinc_const::panic::VALIDATED_DURING_RUNTIME_EXECUTION),
    }

    let provider = zksync::Provider::new(query.network);

    log::debug!("Sending the transfers to ZkSync");
    let mut handles = Vec::with_capacity(output.transfers.len() + 1);

    log::debug!(
        "Sending {} ETH from {} to {}",
        body.transfer.amount.to_string(),
        serde_json::to_string(&body.transfer.from).expect(zinc_const::panic::DATA_CONVERSION),
        serde_json::to_string(&body.transfer.to).expect(zinc_const::panic::DATA_CONVERSION),
    );
    let client_transfer_handle = provider
        .send_tx(
            FranklinTx::Transfer(Box::new(body.transfer)),
            Some(body.signature),
        )
        .await
        .map(|tx_hash| SyncTransactionHandle::new(tx_hash, provider.clone()))
        .map_err(Error::ZkSync)?;
    handles.push(client_transfer_handle);

    let wallet_credentials =
        zksync::WalletCredentials::from_eth_pk(contract.eth_address, contract.eth_private_key)
            .map_err(Error::ZkSync)?;
    let wallet = zksync::Wallet::new(provider, wallet_credentials)
        .await
        .map_err(Error::ZkSync)?;

    for transfer in output.transfers.into_iter() {
        let to: H160 = transfer.to.into();

        log::debug!(
            "Sending {} ETH from {} to {}",
            transfer.amount.to_string(),
            serde_json::to_string(&contract.eth_address).expect(zinc_const::panic::DATA_CONVERSION),
            serde_json::to_string(&to).expect(zinc_const::panic::DATA_CONVERSION),
        );

        let handle = wallet
            .start_transfer()
            .to(to)
            .token("ETH")
            .map_err(Error::ZkSync)?
            .amount(num_old::BigUint::from_bytes_be(
                transfer.amount.to_bytes_be().as_slice(), // TODO: remove when the SDK is updated
            ))
            .send()
            .await
            .map_err(Error::ZkSync)?;

        handles.push(handle);
    }

    log::debug!("Waiting for the transfers to be committed");
    let mut reasons = HashMap::with_capacity(handles.len());
    for handle in handles.into_iter() {
        let tx_info = handle.wait_for_commit().await.map_err(Error::ZkSync)?;

        assert!(
            tx_info.executed,
            "Transaction must be executed after waiting for commit"
        );
        if !tx_info.success.unwrap_or_default() {
            reasons.insert(
                handle.hash(),
                tx_info
                    .fail_reason
                    .unwrap_or_else(|| "Unknown reason".to_owned()),
            );
        }
    }

    if !reasons.is_empty() {
        log::debug!("Reporting {} transfer failures", reasons.len());
        return Err(Error::TransferFailure { reasons });
    }

    log::debug!("Committing the contract storage state");
    app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql_client
        .update_fields(storage_fields)
        .await
        .map_err(Error::Database)?;

    let response = output.result.into_json();

    log::debug!("The sequence has been successfully executed");
    Ok(Response::new_with_data(StatusCode::OK, response))
}
