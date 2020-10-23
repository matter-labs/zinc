//!
//! The contract resource PUT method `query` module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use serde_json::json;
use serde_json::Value as JsonValue;

use zinc_build::Value as BuildValue;
use zinc_vm::Bn256;
use zinc_vm::ContractInput;
use zinc_zksync::TransactionMsg;

use crate::database::model::field::select::Input as FieldSelectInput;
use crate::response::Response;
use crate::shared_data::SharedData;
use crate::storage::Storage;

use self::error::Error;
use self::request::Body as RequestBody;
use self::request::Query as RequestQuery;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Get the contract from the in-memory cache.
/// 2. Get the contract storage from data sources and convert it to the Zinc VM representation.
/// 3. If the method was not specified, return the contract storage to the client.
/// 4. Extract the called method from the contract metadata and check if it is immutable.
/// 5. Parse the method input arguments.
/// 6. Run the method on the Zinc VM.
/// 7. Send the contract method execution result back to the client.
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
        .postgresql
        .clone();

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
    let account_id = contract.account_id.ok_or_else(|| {
        Error::ContractLocked(
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
    let wallet = zksync::Wallet::new(provider, wallet_credentials).await?;

    log::debug!("Loading the contract storage");
    let database_fields = postgresql
        .select_fields(FieldSelectInput::new(account_id))
        .await?;
    let storage = Storage::new_with_data(
        database_fields,
        contract.build.storage.as_slice(),
        contract.eth_address,
        &wallet,
    )
    .await?;

    let method_name = match query.method {
        Some(method_name) => {
            log::debug!(
                "Querying method `{}` of the contract {}",
                method_name,
                serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION)
            );
            method_name
        }
        None => {
            log::debug!(
                "Querying the storage of the contract {}",
                serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION)
            );
            return Ok(Response::new_with_data(
                StatusCode::OK,
                storage.into_public_build().into_json(),
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
    let input_value =
        BuildValue::try_from_typed_json(arguments, method.input).map_err(Error::InvalidInput)?;

    log::debug!("Running the contract method on the virtual machine");
    let vm_time = std::time::Instant::now();
    let output = async_std::task::spawn_blocking(move || {
        zinc_vm::ContractFacade::new(contract.build).run::<Bn256>(ContractInput::new(
            input_value,
            storage.into_build(),
            method_name,
            TransactionMsg::default(),
        ))
    })
    .await
    .map_err(Error::RuntimeError)?;
    log::debug!("VM executed in {} ms", vm_time.elapsed().as_millis());

    let response = json!({
        "output": output.result.into_json(),
    });

    log::debug!("The query has been successfully executed");
    Ok(Response::new_with_data(StatusCode::OK, response))
}
