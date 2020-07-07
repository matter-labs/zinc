//!
//! The program run feature POST method module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;

use zinc_bytecode::TemplateValue as BytecodeTemplateValue;
use zinc_vm::Bn256;
use zinc_vm::IFacade;

use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Body;
use self::request::Query;

///
/// The program run feature POST method endpoint handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<Query>,
    body: web::Json<Body>,
) -> impl Responder {
    let query = query.into_inner();
    let body = body.into_inner();

    let entry = match app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .get_program_entry(query.name.as_str(), query.entry.as_str())
    {
        Some(entry) => entry,
        None => return Response::new_error(Error::NotFound),
    };

    let input = match BytecodeTemplateValue::from_typed_json(body.input, entry.input_type) {
        Ok(input) => input,
        Err(error) => return Response::new_error(Error::InputError(error)),
    };

    let mongo_client = app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .mongodb_client
        .clone();

    let output = match entry.program.run::<Bn256>(input, Some(mongo_client)) {
        Ok(output) => output.into_json(),
        Err(error) => return Response::new_error(Error::RuntimeError(error)),
    };

    Response::new_success_with_data(StatusCode::OK, output)
}
