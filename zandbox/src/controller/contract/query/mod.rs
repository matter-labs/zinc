//!
//! The contract resource PUT query method module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;
use serde_json::Value as JsonValue;

use zinc_build::Value as BuildValue;
use zinc_vm::Bn256;

use crate::database::model::field::select::input::Input as FieldSelectInput;
use crate::database::model::field::select::output::Output as FieldSelectOutput;
use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Body;
use self::request::Query;

///
/// The HTTP request handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<Query>,
    body: web::Json<Body>,
) -> impl Responder {
    let query = query.into_inner();
    let body = body.into_inner();

    let contract = match app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .contracts
        .get(&query.contract_id)
        .cloned()
    {
        Some(contract) => contract,
        None => return Response::error(Error::ContractNotFound),
    };

    let storage_value = match app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .postgresql_client
        .select_fields(FieldSelectInput::new(query.contract_id))
        .await
    {
        Ok(output) => {
            if output.len() != contract.storage.len() {
                return Response::error(Error::InvalidStorageSize {
                    expected: contract.storage.len(),
                    found: output.len(),
                });
            }

            let mut fields = Vec::with_capacity(output.len());
            for (index, FieldSelectOutput { name, value }) in output.into_iter().enumerate() {
                let r#type = contract.storage[index].1.clone();
                let value = match BuildValue::try_from_typed_json(value, r#type) {
                    Ok(value) => value,
                    Err(error) => return Response::error(Error::InvalidStorage(error)),
                };
                fields.push((name, value))
            }
            BuildValue::Contract(fields)
        }
        Err(error) => return Response::error(Error::Database(error)),
    };

    match query.method {
        Some(method_name) => {
            let method = match contract.methods.get(method_name.as_str()).cloned() {
                Some(method) => method,
                None => return Response::error(Error::MethodNotFound),
            };

            let arguments = match body.arguments {
                Some(arguments) => arguments,
                None => return Response::error(Error::MethodArgumentsNotFound),
            };

            let input_value = match BuildValue::try_from_typed_json(arguments, method.input) {
                Ok(input_value) => input_value,
                Err(error) => return Response::error(Error::InvalidInput(error)),
            };

            if method.is_mutable {
                return Response::error(Error::MethodIsMutable);
            }

            let output = match zinc_vm::ContractFacade::new(contract).run::<Bn256>(
                input_value,
                storage_value,
                method_name,
            ) {
                Ok(output) => output,
                Err(error) => return Response::error(Error::RuntimeError(error)),
            };

            dbg!(output.transfers);

            Response::<JsonValue, Error>::success_with_data(
                StatusCode::OK,
                output.result.into_json(),
            )
        }
        None => Response::<JsonValue, Error>::success_with_data(
            StatusCode::OK,
            storage_value.into_json(),
        ),
    }
}
