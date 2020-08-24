//!
//! The contract resource POST call method module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;

use zinc_build::Program as BuildProgram;
use zinc_build::Value as BuildValue;
use zinc_vm::Bn256;

use crate::database::model::field::select::input::Input as FieldSelectInput;
use crate::database::model::field::select::output::Output as FieldSelectOutput;
use crate::database::model::field::update::input::Input as FieldUpdateInput;
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

    let program = match app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .templates
        .get(&query.template_id)
        .cloned()
    {
        Some(program) => program,
        None => return Response::error(Error::ContractNotFound),
    };

    let contract = match program {
        BuildProgram::Circuit(_circuit) => return Response::error(Error::NotAContract),
        BuildProgram::Contract(contract) => contract,
    };

    let method = match contract.methods.get(query.method.as_str()).cloned() {
        Some(method) => method,
        None => return Response::error(Error::MethodNotFound),
    };
    if !method.is_mutable {
        return Response::error(Error::MethodIsImmutable);
    }

    let input_value = match BuildValue::try_from_typed_json(body.input, method.input) {
        Ok(input_value) => input_value,
        Err(error) => return Response::error(Error::InvalidInput(error)),
    };

    let storage_fields_count = contract.storage.len();
    let storage_value = match app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .postgresql_client
        .select_fields(FieldSelectInput::new(query.account_id))
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

    let storage = match zinc_vm::ContractFacade::new(contract).run::<Bn256>(
        input_value,
        storage_value,
        query.method,
    ) {
        Ok((_output, storage)) => storage,
        Err(error) => return Response::error(Error::RuntimeError(error)),
    };

    let mut storage_fields = Vec::with_capacity(storage_fields_count);
    match storage {
        BuildValue::Contract(fields) => {
            for (index, (_name, value)) in fields.into_iter().enumerate() {
                let value = value.into_json();
                storage_fields.push(FieldUpdateInput::new(index as i16, query.account_id, value));
            }
        }
        _ => panic!(zinc_const::panic::VALIDATED_DURING_RUNTIME_EXECUTION),
    }

    if let Err(error) = app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .postgresql_client
        .update_fields(storage_fields)
        .await
    {
        return Response::error(Error::Database(error));
    }

    Response::<(), Error>::success(StatusCode::OK)
}
