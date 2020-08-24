//!
//! The contract resource POST method module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;
use hex::FromHex;

use zinc_build::Program as BuildProgram;
use zinc_build::Type as BuildType;
use zinc_build::Value as BuildValue;
use zinc_vm::Bn256;

use crate::database::model::contract::insert::input::Input as ContractInsertInput;
use crate::database::model::field::insert::input::Input as FieldInsertInput;
use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Body;
use self::request::Query;

static CONSTRUCTOR_NAME: &str = "new";

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
    let constructor = match contract.methods.get(CONSTRUCTOR_NAME).cloned() {
        Some(constructor) => constructor,
        None => return Response::error(Error::ConstructorNotFound),
    };
    let input_value = match BuildValue::try_from_typed_json(body.input, constructor.input) {
        Ok(input_value) => input_value,
        Err(error) => return Response::error(Error::InvalidInput(error)),
    };
    let storage_fields_count = contract.storage.len();
    let storage_value = BuildValue::new(BuildType::Contract(contract.storage.clone()));
    let output = match zinc_vm::ContractFacade::new(contract).run::<Bn256>(
        input_value,
        storage_value,
        CONSTRUCTOR_NAME.to_owned(),
    ) {
        Ok((output, _storage)) => output,
        Err(error) => return Response::error(Error::RuntimeError(error)),
    };

    let eth_address = match <[u8; 20]>::from_hex(body.eth_address) {
        Ok(eth_address) => eth_address.to_vec(),
        Err(error) => return Response::error(Error::InvalidAddress(error)),
    };
    if let Err(error) = app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .postgresql_client
        .insert_contract(ContractInsertInput::new(
            query.account_id,
            query.template_id,
            eth_address,
        ))
        .await
    {
        return Response::error(Error::Database(error));
    }

    let mut storage_fields = Vec::with_capacity(storage_fields_count);
    match output {
        BuildValue::Structure(mut fields) => match fields.remove(0).1 {
            BuildValue::Contract(fields) => {
                for (index, (name, value)) in fields.into_iter().enumerate() {
                    let value = value.into_json();
                    storage_fields.push(FieldInsertInput::new(
                        index as i16,
                        query.account_id,
                        name,
                        value,
                    ));
                }
            }
            _ => return Response::error(Error::InvalidStorage),
        },
        _ => return Response::error(Error::InvalidStorage),
    }
    if let Err(error) = app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .postgresql_client
        .insert_fields(storage_fields)
        .await
    {
        return Response::error(Error::Database(error));
    }

    Response::<(), Error>::success(StatusCode::CREATED)
}
