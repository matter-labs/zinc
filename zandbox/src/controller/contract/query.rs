//!
//! The contract resource PUT method `query` module.
//!

use actix_web::http::StatusCode;
use actix_web::web;
use num::BigInt;

use crate::contract::Contract;
use crate::error::Error;
use crate::response::Response;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Get the contract and its data from the database.
/// 2. If the method was not specified, return the contract storage to the client.
/// 3. Extract the called method from the contract metadata and check if it is immutable.
/// 4. Parse the method input arguments.
/// 5. Run the method on the VM.
/// 6. Send the contract method execution result back to the client.
///
pub async fn handle(
    app_data: crate::WebData,
    query: web::Query<zinc_types::QueryRequestQuery>,
    body: web::Json<zinc_types::QueryRequestBody>,
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

    let contract = Contract::new(network, postgresql.clone(), query.address).await?;

    let method_name = match query.method {
        Some(method_name) => {
            log::info!("[{}] Querying method `{}`", log_id, method_name);
            method_name
        }
        None => {
            log::info!("[{}] Querying the storage", log_id);
            return Ok(Response::new_with_data(
                StatusCode::OK,
                contract.storage.into_public_build().into_json(),
            ));
        }
    };

    let method = match contract.build.methods.get(method_name.as_str()).cloned() {
        Some(method) => method,
        None => return Err(Error::MethodNotFound(method_name)),
    };
    if method.is_mutable {
        return Err(Error::MethodIsMutable(method_name));
    }

    let arguments = match body.arguments {
        Some(arguments) => arguments,
        None => return Err(Error::MethodArgumentsNotFound(method_name)),
    };
    let eth_address_bigint =
        BigInt::from_bytes_be(num::bigint::Sign::Plus, contract.eth_address.as_bytes());
    let mut arguments = zinc_types::Value::try_from_typed_json(arguments, method.input)
        .map_err(Error::InvalidInput)?;
    arguments.insert_contract_instance(eth_address_bigint.clone());

    let output = contract
        .run_method(
            method_name,
            zinc_types::TransactionMsg::default(),
            arguments,
            postgresql,
        )
        .await?;

    let response = serde_json::json!({
        "output": output.result.into_json(),
    });

    log::info!("[{}] Query finished", log_id);
    Ok(Response::new_with_data(StatusCode::OK, response))
}
