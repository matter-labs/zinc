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

use zinc_build::ContractFieldValue as BuildContractFieldValue;
use zinc_build::Value as BuildValue;
use zinc_vm::Bn256;

use crate::database::model::field::select::input::Input as FieldSelectInput;
use crate::database::model::field::select::output::Output as FieldSelectOutput;
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
/// 2. Get the contract storage from the database and convert it to the Zinc VM representation.
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
        .postgresql_client
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

    log::debug!("Loading the contract storage");
    let storage_value = postgresql
        .select_fields(FieldSelectInput::new(query.address))
        .await?;
    let mut contract_fields = Vec::with_capacity(storage_value.len());
    for (index, FieldSelectOutput { name, value }) in storage_value.into_iter().enumerate() {
        let r#type = contract.build.storage[index].r#type.clone();
        let value = BuildValue::try_from_typed_json(value, r#type)
            .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);
        contract_fields.push(BuildContractFieldValue::new(
            name,
            value,
            contract.build.storage[index].is_public,
            contract.build.storage[index].is_external,
        ));
    }

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
                BuildValue::Contract(
                    contract_fields
                        .into_iter()
                        .filter(|field| field.is_public)
                        .collect(),
                )
                .into_json(),
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
    let output = async_std::task::spawn_blocking(move || {
        zinc_vm::ContractFacade::new(contract.build).run::<Bn256>(
            input_value,
            BuildValue::Contract(contract_fields),
            method_name,
        )
    })
    .await
    .map_err(Error::RuntimeError)?;

    let response = json!({
        "output": output.result.into_json(),
    });

    log::debug!("The query has been successfully executed");
    Ok(Response::new_with_data(StatusCode::OK, response))
}
