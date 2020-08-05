//!
//! The program resource POST method module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;

use zinc_bytecode::DataType;
use zinc_compiler::Source;
use zinc_compiler::State;
use zinc_postgres::EntryInsertInput;
use zinc_postgres::ProgramInsertInput;

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

    let source = match Source::try_from_string(body.source.clone(), true)
        .map_err(|error| error.to_string())
    {
        Ok(source) => source,
        Err(error) => return Response::error(Error::Compiler(error)),
    };

    let compiled = match source
        .compile(query.name.clone())
        .map_err(|error| error.to_string())
    {
        Ok(bytecode) => State::unwrap_rc(bytecode),
        Err(error) => return Response::error(Error::Compiler(error)),
    };

    let source = serde_json::to_value(body.source).expect(zinc_const::panic::DATA_SERIALIZATION);

    let storage_type = match compiled.contract_storage() {
        Some(contract_storage) => DataType::Contract(contract_storage),
        None => DataType::Contract(vec![]),
    };

    match app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .postgresql_client
        .insert_program(ProgramInsertInput::new(
            query.name,
            query.version,
            source,
            serde_json::to_value(storage_type).expect(zinc_const::panic::DATA_SERIALIZATION),
            vec![],
            vec![],
        ))
        .await
    {
        Ok(program_id) => {
            let entries = app_data
                .write()
                .expect(zinc_const::panic::MUTEX_SYNC)
                .append_programs(program_id, compiled);

            let entries: Vec<EntryInsertInput> = entries
                .into_iter()
                .map(|(name, entry)| {
                    EntryInsertInput::new(
                        program_id,
                        name,
                        false,
                        serde_json::to_value(&entry.input())
                            .expect(zinc_const::panic::DATA_SERIALIZATION),
                        serde_json::to_value(&entry.output())
                            .expect(zinc_const::panic::DATA_SERIALIZATION),
                    )
                })
                .collect();
            if let Err(error) = app_data
                .read()
                .expect(zinc_const::panic::MUTEX_SYNC)
                .postgresql_client
                .insert_entries(entries)
                .await
            {
                return Response::error(Error::Postgresql(error));
            }
        }
        Err(error) => return Response::error(Error::Postgresql(error)),
    }

    Response::<(), Error>::success(StatusCode::CREATED)
}
