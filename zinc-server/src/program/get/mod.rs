//!
//! The program resource DELETE method module.
//!

pub mod error;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;

use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;

///
/// The HTTP request handler.
///
pub async fn handle(app_data: web::Data<Arc<RwLock<SharedData>>>) -> impl Responder {
    let programs = app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .get_programs();

    Response::<Vec<String>, Error>::success_with_data(StatusCode::OK, programs)
}
