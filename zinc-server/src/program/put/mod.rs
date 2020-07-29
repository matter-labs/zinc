//!
//! The program resource PUT method module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;

use zinc_bytecode::DataType;
use zinc_bytecode::TemplateValue;
use zinc_compiler::Source;
use zinc_compiler::State;
use zinc_mongo::Storage as MongoStorage;

use crate::response::Response;
use crate::shared_data::program::Program;
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

    let exists = app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .contains_program(query.name.as_str());

    let source = match Source::try_from_string(body.source.clone(), true)
        .map_err(|error| error.to_string())
    {
        Ok(source) => source,
        Err(error) => return Response::error(Error::Compiler(error)),
    };

    let bytecode = match source
        .compile(query.name.clone())
        .map_err(|error| error.to_string())
    {
        Ok(bytecode) => State::unwrap_rc(bytecode),
        Err(error) => return Response::error(Error::Compiler(error)),
    };

    let source = bson::to_bson(&body.source).expect(zinc_const::panic::DATA_SERIALIZATION);

    let (program, record) = match bytecode.contract_storage() {
        Some(contract_storage) => {
            let r#type = DataType::Contract(contract_storage.clone());
            let storage =
                MongoStorage::from_bson(TemplateValue::new(r#type).into_bson()).into_bson();
            let record = bson::doc! {
                "source": source,
                "storage": storage,
            };

            let program = Program::new_contract(
                body.source,
                Program::from_bytecode(bytecode),
                contract_storage,
            );

            (program, record)
        }
        None => {
            let record = bson::doc! {
                "source": source,
            };

            let program = Program::new_circuit(body.source, Program::from_bytecode(bytecode));

            (program, record)
        }
    };

    let entries: Vec<String> = program.entries.keys().cloned().collect();

    if let Err(error) = app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .mongodb_client
        .rewrite_collection(query.name.as_str(), record)
        .await
    {
        return Response::error(Error::MongoDb(error));
    }

    app_data
        .write()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .insert_program(query.name, program);

    let status_code = if exists {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::CREATED
    };

    Response::success_with_data(status_code, entries)
}
