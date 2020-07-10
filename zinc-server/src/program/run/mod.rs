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

use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue as BytecodeTemplateValue;
use zinc_vm::Bn256;
use zinc_vm::CircuitFacade;
use zinc_vm::ContractFacade;

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
        None => return Response::error(Error::NotFound),
    };

    let input = match BytecodeTemplateValue::try_from_typed_json(body.input, entry.input_type) {
        Ok(input) => input,
        Err(error) => return Response::error(Error::InputError(error)),
    };

    let output = match entry.program {
        BytecodeProgram::Circuit(circuit) => {
            match CircuitFacade::new(circuit).run::<Bn256>(input) {
                Ok(output) => output,
                Err(error) => return Response::error(Error::RuntimeError(error)),
            }
        }
        BytecodeProgram::Contract(contract) => {
            let mongo_client = app_data
                .read()
                .expect(zinc_const::panic::MUTEX_SYNC)
                .mongodb_client
                .to_owned();

            let storage = match mongo_client.get_storage(query.name.as_str()).await {
                Ok(storage) => storage,
                Err(error) => return Response::error(Error::MongoDb(error)),
            };

            let (output, storage) =
                match ContractFacade::new(contract).run::<Bn256>(input, Some(storage)) {
                    Ok((output, storage)) => (output, storage),
                    Err(error) => return Response::error(Error::RuntimeError(error)),
                };

            if let Err(error) = mongo_client
                .update_storage(query.name.as_str(), storage)
                .await
            {
                return Response::error(Error::MongoDb(error));
            }

            output
        }
    };

    Response::success_with_data(StatusCode::OK, output)
}
