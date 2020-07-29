//!
//! The program input template resource GET method module.
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
/// The HTTP request handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<Query>,
) -> impl Responder {
    let query = query.into_inner();

    let template = app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .get_program_entry_input_template(query.name.as_str(), query.entry.as_str());

    match template {
        Some(template) => Response::success_with_data(StatusCode::OK, template),
        None => Response::error(Error::NotFound),
    }
}
