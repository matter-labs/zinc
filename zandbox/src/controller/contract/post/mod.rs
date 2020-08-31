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
use zinc_compiler::Source;
use zinc_compiler::State;
use zinc_vm::Bn256;

use crate::database::model::contract::insert::input::Input as ContractInsertInput;
use crate::database::model::field::insert::input::Input as FieldInsertInput;
use crate::database::model::method::insert::input::Input as MethodInsertInput;
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

    let source = match Source::try_from_string(body.source.clone(), true)
        .map_err(|error| error.to_string())
    {
        Ok(source) => source,
        Err(error) => return Response::error(Error::Compiler(error)),
    };

    let state = match source
        .compile(query.name.clone())
        .map_err(|error| error.to_string())
    {
        Ok(state) => State::unwrap_rc(state),
        Err(error) => return Response::error(Error::Compiler(error)),
    };

    let contract = match state.into_program(true) {
        BuildProgram::Circuit(_circuit) => return Response::error(Error::NotAContract),
        BuildProgram::Contract(contract) => contract,
    };

    app_data
        .write()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .contracts
        .insert(query.contract_id, contract.clone());

    let constructor = match contract.methods.get(CONSTRUCTOR_NAME).cloned() {
        Some(constructor) => constructor,
        None => return Response::error(Error::ConstructorNotFound),
    };

    let methods: Vec<MethodInsertInput> = contract
        .methods
        .iter()
        .map(|(name, method)| {
            MethodInsertInput::new(
                query.contract_id,
                name.to_owned(),
                false,
                serde_json::to_value(&method.input).expect(zinc_const::panic::DATA_SERIALIZATION),
                serde_json::to_value(&method.output).expect(zinc_const::panic::DATA_SERIALIZATION),
            )
        })
        .collect();

    let input_value =
        match BuildValue::try_from_typed_json(body.constructor_input, constructor.input) {
            Ok(input_value) => input_value,
            Err(error) => return Response::error(Error::InvalidInput(error)),
        };

    let storage = contract.storage.clone();
    let storage_value = BuildValue::new(BuildType::Contract(contract.storage.clone()));
    let output = match zinc_vm::ContractFacade::new(contract).run::<Bn256>(
        input_value,
        storage_value,
        CONSTRUCTOR_NAME.to_owned(),
    ) {
        Ok((output, _storage)) => output,
        Err(error) => return Response::error(Error::RuntimeError(error)),
    };

    let eth_address = match <[u8; 20]>::from_hex("0000000000000000000000000000000000000000") {
        Ok(eth_address) => eth_address.to_vec(),
        Err(error) => return Response::error(Error::InvalidAddress(error)),
    };

    let mut fields = Vec::with_capacity(storage.len());
    match output {
        BuildValue::Structure(mut storage_fields) => match storage_fields.remove(0).1 {
            BuildValue::Contract(storage_fields) => {
                for (index, (name, value)) in storage_fields.into_iter().enumerate() {
                    let value = value.into_json();
                    fields.push(FieldInsertInput::new(
                        query.contract_id,
                        index as i16,
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
        .insert_contract(ContractInsertInput::new(
            query.contract_id,
            query.name,
            query.version,
            serde_json::to_value(body.source).expect(zinc_const::panic::DATA_SERIALIZATION),
            vec![],
            serde_json::to_value(BuildType::Contract(storage))
                .expect(zinc_const::panic::DATA_SERIALIZATION),
            vec![],
            eth_address,
        ))
        .await
    {
        return Response::error(Error::Database(error));
    }

    if let Err(error) = app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .postgresql_client
        .insert_methods(methods)
        .await
    {
        return Response::error(Error::Database(error));
    }

    if let Err(error) = app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .postgresql_client
        .insert_fields(fields)
        .await
    {
        return Response::error(Error::Database(error));
    }

    Response::<(), Error>::success(StatusCode::CREATED)
}
