//!
//! The program resource DELETE method module.
//!

pub mod request;
pub mod response;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::web;
use actix_web::Responder;

use crate::app_data::AppData;

use self::request::Request;
use self::response::Response;

///
/// The program DELETE method endpoint handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<AppData>>>,
    request: web::Query<Request>,
) -> impl Responder {
    let request = request.into_inner();

    let source = app_data
        .write()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .remove_program(request.name.as_str());

    match source {
        Some(source) => web::Json(Response::new_success(source)),
        None => web::Json(Response::new_error("Not found".to_owned())),
    }
}
