//!
//! The contract method templates resource GET method module.
//!

pub mod error;
pub mod request;
pub mod response;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;

use crate::database::model::method::select::types::Input as MethodSelectTypesInput;
use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Query;
use self::response::Body;

///
/// The HTTP request handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<Query>,
) -> impl Responder {
    let query = query.into_inner();

    let body: Body = match app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .postgresql_client
        .select_method_types(MethodSelectTypesInput::new(query.contract_id, query.name))
        .await
    {
        Ok(output) => output.into(),
        Err(error) => return Response::error(Error::Database(error)),
    };

    Response::success_with_data(StatusCode::OK, body)
}
