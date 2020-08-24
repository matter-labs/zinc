//!
//! The template resource POST method module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;

use zinc_build::Program as BuildProgram;
use zinc_build::Type as BuildType;
use zinc_compiler::Source;
use zinc_compiler::State;

use crate::database::model::method::insert::input::Input as MethodInsertInput;
use crate::database::model::template::insert::input::Input as TemplateInsertInput;
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

    let state = match source
        .compile(query.name.clone())
        .map_err(|error| error.to_string())
    {
        Ok(bytecode) => State::unwrap_rc(bytecode),
        Err(error) => return Response::error(Error::Compiler(error)),
    };

    let storage_type = match state.contract_storage() {
        Some(contract_storage) => BuildType::Contract(contract_storage),
        None => return Response::error(Error::NotAContract),
    };

    if let Err(error) = app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .postgresql_client
        .insert_template(TemplateInsertInput::new(
            query.account_id,
            query.name,
            query.version,
            vec![],
            serde_json::to_value(storage_type).expect(zinc_const::panic::DATA_SERIALIZATION),
            vec![],
        ))
        .await
    {
        return Response::error(Error::Database(error));
    }

    let program = state.into_program(true);
    app_data
        .write()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .templates
        .insert(query.account_id, program.clone());

    let template_id = query.account_id;
    let methods: Vec<MethodInsertInput> = match program {
        BuildProgram::Circuit(_) => return Response::error(Error::NotAContract),
        BuildProgram::Contract(contract) => contract
            .methods
            .into_iter()
            .map(|(name, method)| {
                MethodInsertInput::new(
                    template_id,
                    name,
                    false,
                    serde_json::to_value(&method.input)
                        .expect(zinc_const::panic::DATA_SERIALIZATION),
                    serde_json::to_value(&method.output)
                        .expect(zinc_const::panic::DATA_SERIALIZATION),
                )
            })
            .collect(),
    };

    if let Err(error) = app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .postgresql_client
        .insert_methods(methods)
        .await
    {
        return Response::error(Error::Database(error));
    }

    Response::<(), Error>::success(StatusCode::CREATED)
}
