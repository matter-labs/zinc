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

use crate::database::model::method::select::types::Input as MethodSelectTypesInput;
use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Query as RequestQuery;
use self::response::Body as ResponseBody;

///
/// The HTTP request handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<RequestQuery>,
) -> crate::Result<ResponseBody, Error> {
    let query = query.into_inner();

    let body: ResponseBody = app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .postgresql_client
        .select_method_types(MethodSelectTypesInput::new(query.contract_id, query.name))
        .await
        .map_err(Error::Database)?
        .into();

    Response::new_with_data(StatusCode::OK, body).into()
}
