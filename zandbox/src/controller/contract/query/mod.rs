//!
//! The contract resource PUT query method module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use serde_json::Value as JsonValue;

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
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<RequestQuery>,
    body: web::Json<RequestBody>,
) -> crate::Result<JsonValue, Error> {
    let query = query.into_inner();
    let body = body.into_inner();

    let contract = app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .contracts
        .get(&query.account_id)
        .cloned()
        .ok_or(Error::ContractNotFound)?;

    log::debug!("Loading the contract storage");
    let storage_value = app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .postgresql_client
        .select_fields(FieldSelectInput::new(query.account_id as i64))
        .await
        .map_err(Error::Database)?;
    if storage_value.len() != contract.build.storage.len() {
        return Err(Error::InvalidStorageSize {
            expected: contract.build.storage.len(),
            found: storage_value.len(),
        });
    }
    let mut fields = Vec::with_capacity(storage_value.len());
    for (index, FieldSelectOutput { name, value }) in storage_value.into_iter().enumerate() {
        let r#type = contract.build.storage[index].1.clone();
        let value = match BuildValue::try_from_typed_json(value, r#type) {
            Ok(value) => value,
            Err(error) => return Err(Error::InvalidStorage(error)),
        };
        fields.push((name, value))
    }
    let storage_value = BuildValue::Contract(fields);

    let method_name = match query.method {
        Some(method_name) => {
            log::debug!(
                "Querying method `{}` of the contract #{}",
                method_name,
                query.account_id
            );
            method_name
        }
        None => {
            log::debug!("Querying the storage of the contract #{}", query.account_id);
            return Ok(Response::new_with_data(
                StatusCode::OK,
                storage_value.into_json(),
            ));
        }
    };

    let method = contract
        .build
        .methods
        .get(method_name.as_str())
        .cloned()
        .ok_or(Error::MethodNotFound)?;
    let arguments = body.arguments.ok_or(Error::MethodArgumentsNotFound)?;
    let input_value =
        BuildValue::try_from_typed_json(arguments, method.input).map_err(Error::InvalidInput)?;
    if method.is_mutable {
        return Err(Error::MethodIsMutable);
    }

    log::debug!("Running the contract method on the virtual machine");
    let output = async_std::task::spawn_blocking(move || {
        zinc_vm::ContractFacade::new(contract.build).run::<Bn256>(
            input_value,
            storage_value,
            method_name,
        )
    })
    .await
    .map_err(Error::RuntimeError)?;

    let response = output.result.into_json();

    log::debug!("The sequence has been successfully executed");
    Ok(Response::new_with_data(StatusCode::OK, response))
}
