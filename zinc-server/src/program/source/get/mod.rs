//!
//! The program source resource GET method module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;

use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Query;

///
/// The program source resource GET method endpoint handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<Query>,
) -> impl Responder {
    let query = query.into_inner();

    let source = app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .get_program_source(query.name.as_str());

    match source {
        Some(source) => Response::success_with_data(StatusCode::OK, source),
        None => Response::error(Error::NotFound),
    }
}
