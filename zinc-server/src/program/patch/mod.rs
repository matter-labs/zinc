//!
//! The program resource PATCH method module.
//!

pub mod request;
pub mod response;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use actix_web::web;
use actix_web::Responder;

use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue as BytecodeTemplateValue;
use zinc_compiler::Bytecode;
use zinc_compiler::Source;

use crate::app_data::program::entry::Entry;
use crate::app_data::program::Program;
use crate::app_data::AppData;

use self::request::Request;
use self::response::Response;

///
/// The program PATCH method endpoint handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<AppData>>>,
    request: web::Json<Request>,
) -> impl Responder {
    let request = request.into_inner();

    let source = match Source::try_from_string(request.source.clone(), true)
        .map_err(|error| error.to_string())
    {
        Ok(source) => source,
        Err(error) => return web::Json(Response::new_error(error)),
    };

    let bytecode = match source.compile().map_err(|error| error.to_string()) {
        Ok(bytecode) => Bytecode::unwrap_rc(bytecode),
        Err(error) => return web::Json(Response::new_error(error)),
    };

    let entries: HashMap<String, Entry> = bytecode
        .into_entries()
        .into_iter()
        .map(|(name, entry)| {
            let program = BytecodeProgram::from_bytes(entry.into_bytecode().as_slice())
                .expect(zinc_const::panic::DATA_SERIALIZATION);
            let input_template = BytecodeTemplateValue::new(program.input())
                .try_into_json()
                .unwrap();
            let output_template = BytecodeTemplateValue::new(program.output())
                .try_into_json()
                .unwrap();
            let entry = Entry::new(program, input_template, output_template);

            (name, entry)
        })
        .collect();

    app_data
        .write()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .insert_program(request.name, Program::new(request.source, entries));

    web::Json(Response::new_success())
}
