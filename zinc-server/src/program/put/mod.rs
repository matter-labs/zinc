//!
//! The program resource PUT method module.
//!

pub mod error;
pub mod request;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;

use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue as BytecodeTemplateValue;
use zinc_compiler::Bytecode;
use zinc_compiler::Source;

use crate::response::Response;
use crate::shared_data::program::entry::Entry;
use crate::shared_data::program::Program;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Body;
use self::request::Query;

///
/// The program resource PUT method endpoint handler.
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
        .contains(query.name.as_str());

    let source = match Source::try_from_string(body.source.clone(), true)
        .map_err(|error| error.to_string())
    {
        Ok(source) => source,
        Err(error) => return Response::new_error(Error::Compiling(error)),
    };

    let bytecode = match source.compile().map_err(|error| error.to_string()) {
        Ok(bytecode) => Bytecode::unwrap_rc(bytecode),
        Err(error) => return Response::new_error(Error::Compiling(error)),
    };

    let entries: HashMap<String, Entry> = bytecode
        .into_entries()
        .into_iter()
        .map(|(name, entry)| {
            let program = BytecodeProgram::from_bytes(entry.into_bytecode().as_slice())
                .expect(zinc_const::panic::DATA_SERIALIZATION);

            let input_type = program.input();
            let input_template = BytecodeTemplateValue::new(input_type.clone()).into_json();

            let output_type = program.output();
            let output_template = BytecodeTemplateValue::new(output_type.clone()).into_json();

            let entry = Entry::new(
                program,
                input_type,
                input_template,
                output_type,
                output_template,
            );

            (name, entry)
        })
        .collect();

    app_data
        .write()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .insert_program(query.name, Program::new(body.source, entries));

    Response::<(), _>::new_success(if exists {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::CREATED
    })
}
