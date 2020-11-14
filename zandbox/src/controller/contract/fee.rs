//!
//! The contract resource PUT method `fee` module.
//!

use actix_web::http::StatusCode;
use actix_web::web;
use num_old::BigUint;
use num_old::Zero;

use zksync_types::tx::ZkSyncTx;
use zksync_types::TxFeeTypes;

use zinc_vm::Bn256;
use zinc_vm::ContractInput;

use crate::contract::Contract;
use crate::error::Error;
use crate::response::Response;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Get the contract and its data from the database.
/// 2. Extract the called method from its metadata and check if it is mutable.
/// 3. Parse the method input arguments.
/// 4. Run the method on the Zinc VM.
/// 5. Calculate the fee required for the transfers.
/// 6. Send the calculated fee back to the client.
///
pub async fn handle(
    app_data: crate::WebData,
    query: web::Query<zinc_zksync::FeeRequestQuery>,
    body: web::Json<zinc_zksync::FeeRequestBody>,
) -> crate::Result<zinc_zksync::FeeResponseBody, Error> {
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

    log::info!(
        "[{}] Calculating the fee for method `{}`",
        log_id,
        query.method,
    );

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

    let mut fee = BigUint::zero();
    let token =
        match body.transaction.tx {
            ZkSyncTx::Transfer(ref transfer) => contract
                .wallet
                .tokens
                .resolve(transfer.token.into())
                .ok_or_else(|| Error::TokenNotFound(transfer.token.to_string()))?,
            _ => panic!(zinc_const::panic::VALUE_ALWAYS_EXISTS),
        };
    for transfer in output.transfers.into_iter() {
        fee += contract
            .wallet
            .provider
            .get_tx_fee(TxFeeTypes::Transfer, transfer.recipient.into(), token.id)
            .await?
            .total_fee;
    }
    log::info!(
        "[{}] The total fee is {} {}",
        log_id,
        zksync_utils::format_units(&fee, token.decimals),
        token.symbol,
    );

    let response = zinc_zksync::FeeResponseBody::new(fee);

    Ok(Response::new_with_data(StatusCode::OK, response))
}
