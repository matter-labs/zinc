//!
//! The contract resource PUT method `fee` module.
//!

use actix_web::http::StatusCode;
use actix_web::web;
use num::BigInt;
use num_old::BigUint;
use num_old::Zero;

use zksync::provider::Provider;

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
/// 4. Run the method on the VM.
/// 5. Calculate the fee required for the initializers and transfers.
/// 6. Send the calculated fee back to the client.
///
pub async fn handle(
    app_data: crate::WebData,
    query: web::Query<zinc_types::FeeRequestQuery>,
    body: web::Json<zinc_types::FeeRequestBody>,
) -> crate::Result<zinc_types::FeeResponseBody, Error> {
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

    let eth_address_bigint =
        BigInt::from_bytes_be(num::bigint::Sign::Plus, contract.eth_address.as_bytes());
    let mut arguments = zinc_types::Value::try_from_typed_json(body.arguments, method.input)
        .map_err(Error::InvalidInput)?;
    arguments.insert_contract_instance(eth_address_bigint.clone());

    let output = contract
        .run_method(
            query.method,
            (&body.transaction).try_to_msg(&contract.wallet)?,
            arguments,
            postgresql,
        )
        .await?;

    let mut fee = BigUint::zero();
    let token = match body.transaction.tx {
        zksync_types::ZkSyncTx::Transfer(ref transfer) => contract
            .wallet
            .tokens
            .resolve(transfer.token.into())
            .ok_or_else(|| Error::TokenNotFound(transfer.token.to_string()))?,
        _ => panic!(zinc_const::panic::VALUE_ALWAYS_EXISTS),
    };
    for initializer in output.initializers.into_iter() {
        fee += contract
            .wallet
            .provider
            .get_tx_fee(
                zksync_types::TxFeeTypes::ChangePubKey {
                    onchain_pubkey_auth: true,
                },
                initializer.eth_address,
                token.id,
            )
            .await?
            .total_fee;
    }
    for transfer in output.transfers.into_iter() {
        fee += contract
            .wallet
            .provider
            .get_tx_fee(
                zksync_types::TxFeeTypes::Transfer,
                transfer.recipient,
                token.id,
            )
            .await?
            .total_fee;
    }
    log::info!(
        "[{}] The total fee is {} {}",
        log_id,
        zksync_utils::format_units(&fee, token.decimals),
        token.symbol,
    );

    let response = zinc_types::FeeResponseBody::new(fee);

    Ok(Response::new_with_data(StatusCode::OK, response))
}
