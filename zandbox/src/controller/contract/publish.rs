//!
//! The contract resource POST method `publish` module.
//!

use actix_web::http::StatusCode;
use actix_web::web;

use crate::error::Error;
use crate::response::Response;
use crate::shared_data::locked_contract::LockedContract;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Parse the contract bytecode from the request.
/// 2. Extract the contract constructor from its metadata.
/// 3. Parse the construtor arguments.
/// 4. Run the construtor on the VM which must return the contract storage.
/// 5. Generate a private key for the contract.
/// 6. Fill the implicit contract storage fields.
/// 7. Write the contract and its storage to the in-memory cache.
/// 8. Return the created contract address to the client.
///
pub async fn handle(
    app_data: crate::WebData,
    query: web::Query<zinc_types::PublishRequestQuery>,
    body: web::Json<zinc_types::PublishRequestBody>,
) -> crate::Result<zinc_types::PublishResponseBody, Error> {
    let query = query.into_inner();
    let body = body.into_inner();
    let log_id = format!("{}-{}/{}", query.name, query.version, query.instance);

    let network = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .network;

    log::info!("[{}] Initializing a locked contract", log_id);

    let pending = LockedContract::new(
        network,
        query.name,
        query.version,
        query.instance,
        body.arguments,
        body.project,
        body.bytecode,
        body.verifying_key,
        query.change_pubkey_fee_token,
    )
    .await?;

    let eth_address = pending.eth_address;

    log::info!(
        "[{}] The contract has got address {} and waits for unlocking with fee {} {}",
        log_id,
        serde_json::to_string(&eth_address).expect(zinc_const::panic::DATA_CONVERSION),
        zksync_utils::format_units(
            &pending.change_pubkey_fee,
            pending.change_pubkey_fee_token.decimals
        ),
        pending.change_pubkey_fee_token.symbol,
    );

    let change_pubkey_fee = pending.change_pubkey_fee.clone();
    app_data
        .write()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .locked_contracts
        .insert(eth_address, pending);

    let response = zinc_types::PublishResponseBody::new(eth_address, change_pubkey_fee);

    Ok(Response::new_with_data(StatusCode::CREATED, response))
}
